use std::{
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use compression_core::{
    analyze_pdf as analyze_pdf_core, compress_pdf as compress_pdf_core, AppSettings,
    CompressionRequest, CompressionResult, CompressionSource, GhostscriptAdapter,
    PartialAppSettings, RecentJobRecord, RecentJobSource,
};
use tauri::State;

use crate::state::PersistentStateHandle;

#[tauri::command]
pub fn analyze_pdf(input_path: String) -> compression_core::AnalyzePdfResult {
    analyze_pdf_core(input_path)
}

#[tauri::command]
pub fn compress_pdf(
    state: State<'_, PersistentStateHandle>,
    request: CompressionRequest,
) -> CompressionResult {
    let result = compress_pdf_core(&request, &GhostscriptAdapter);

    if matches!(
        request.source,
        CompressionSource::Desktop | CompressionSource::FinderAction
    ) {
        let source = match request.source {
            CompressionSource::Desktop => RecentJobSource::Desktop,
            CompressionSource::FinderAction => RecentJobSource::FinderAction,
            CompressionSource::Cli => RecentJobSource::Desktop,
        };

        let record = RecentJobRecord {
            id: format!(
                "{}-{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis(),
                result.duration_ms
            ),
            created_at: chrono_timestamp(),
            source,
            input_path: result.input_path.clone(),
            output_path: result.output_path.clone(),
            original_bytes: result.original_bytes,
            output_bytes: result.output_bytes,
            status: result.status.clone(),
            error_code: result.error.as_ref().map(|error| error.code.clone()),
            duration_ms: result.duration_ms,
        };

        let _ = state.push_recent_job(record);
    }

    result
}

#[tauri::command]
pub fn get_recent_jobs(state: State<'_, PersistentStateHandle>) -> Vec<RecentJobRecord> {
    state.recent_jobs()
}

#[tauri::command]
pub fn clear_recent_jobs(state: State<'_, PersistentStateHandle>) -> Result<(), String> {
    state.clear_recent_jobs()
}

#[tauri::command]
pub fn reveal_in_folder(path: String) -> Result<(), String> {
    reveal_path_in_folder(&path)
}

#[tauri::command]
pub fn open_file(path: String) -> Result<(), String> {
    open_path(&path)
}

#[tauri::command]
pub fn get_settings(state: State<'_, PersistentStateHandle>) -> AppSettings {
    state.settings()
}

#[tauri::command]
pub fn update_settings(
    state: State<'_, PersistentStateHandle>,
    settings: PartialAppSettings,
) -> Result<AppSettings, String> {
    state.update_settings(settings)
}

fn chrono_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{now}")
}

fn open_path(path: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = Command::new("open");
        command.arg(path);
        command
    };

    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = Command::new("cmd");
        command.args(["/C", "start", "", path]);
        command
    };

    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut command = Command::new("xdg-open");
        command.arg(path);
        command
    };

    command
        .status()
        .map_err(|error| error.to_string())
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err("Failed to open file.".to_string())
            }
        })
}

fn reveal_path_in_folder(path: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = Command::new("open");
        command.args(["-R", path]);
        command
    };

    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = Command::new("explorer");
        command.arg("/select,");
        command.arg(path);
        command
    };

    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut command = Command::new("xdg-open");
        let parent = std::path::Path::new(path)
            .parent()
            .map(|value| value.display().to_string())
            .unwrap_or_else(|| ".".to_string());
        command.arg(parent);
        command
    };

    command
        .status()
        .map_err(|error| error.to_string())
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err("Failed to reveal output in file manager.".to_string())
            }
        })
}
