use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    errors::CompressionCoreError,
    models::CompressionWarning,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineOutput {
    pub output_bytes: u64,
    pub warnings: Vec<CompressionWarning>,
}

pub trait CompressionEngine {
    fn compress(
        &self,
        input_path: &Path,
        output_path: &Path,
    ) -> Result<EngineOutput, CompressionCoreError>;
}

#[derive(Debug, Default)]
pub struct GhostscriptAdapter;

impl CompressionEngine for GhostscriptAdapter {
    fn compress(
        &self,
        input_path: &Path,
        output_path: &Path,
    ) -> Result<EngineOutput, CompressionCoreError> {
        let binary = resolve_ghostscript_binary()?;
        let output = Command::new(binary)
            .args([
                "-sDEVICE=pdfwrite",
                "-dCompatibilityLevel=1.4",
                "-dNOPAUSE",
                "-dBATCH",
                "-dSAFER",
                "-q",
                "-dPDFSETTINGS=/ebook",
                "-dDetectDuplicateImages=true",
                "-dCompressFonts=true",
                "-dSubsetFonts=true",
                "-dAutoRotatePages=/None",
            ])
            .arg(format!("-sOutputFile={}", output_path.display()))
            .arg(input_path)
            .output()
            .map_err(map_command_error)?;

        if !output.status.success() {
            return Err(map_ghostscript_failure(&output.stderr));
        }

        let output_bytes = fs::metadata(output_path)
            .map_err(map_command_error)?
            .len();

        Ok(EngineOutput {
            output_bytes,
            warnings: vec![],
        })
    }
}

fn resolve_ghostscript_binary() -> Result<PathBuf, CompressionCoreError> {
    if let Ok(path) = std::env::var("SQUEEEZO_GHOSTSCRIPT_BIN") {
        return Ok(PathBuf::from(path));
    }

    #[cfg(target_os = "macos")]
    {
        for candidate in ["/opt/homebrew/bin/gs", "/usr/local/bin/gs", "/usr/bin/gs"] {
            let path = PathBuf::from(candidate);
            if path.is_file() {
                return Ok(path);
            }
        }
    }

    #[cfg(target_os = "windows")]
    let candidates = ["gswin64c", "gswin32c"];

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let candidates = ["gs"];

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return Err(CompressionCoreError::UnsupportedPlatform);

    for candidate in candidates {
        if let Some(path) = find_in_path(candidate) {
            return Ok(path);
        }
    }

    Err(CompressionCoreError::EngineMissing)
}

fn find_in_path(binary_name: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|path| path.join(binary_name))
            .find(|candidate| candidate.is_file())
    })
}

fn map_command_error(error: std::io::Error) -> CompressionCoreError {
    match error.kind() {
        std::io::ErrorKind::NotFound => CompressionCoreError::EngineMissing,
        std::io::ErrorKind::PermissionDenied => CompressionCoreError::WriteDenied,
        std::io::ErrorKind::StorageFull => CompressionCoreError::OutOfSpace,
        std::io::ErrorKind::WouldBlock => CompressionCoreError::FileInUse,
        _ => CompressionCoreError::Unknown,
    }
}

fn map_ghostscript_failure(stderr: &[u8]) -> CompressionCoreError {
    let stderr = String::from_utf8_lossy(stderr);
    let stderr = stderr.trim();
    let normalized = stderr.to_ascii_lowercase();

    if normalized.contains("password")
        || normalized.contains("invalidfileaccess")
        || normalized.contains("requires a password")
    {
        return CompressionCoreError::PasswordProtected;
    }

    if normalized.contains("permission denied") {
        return CompressionCoreError::WriteDenied;
    }

    if normalized.contains("no space left on device") {
        return CompressionCoreError::OutOfSpace;
    }

    if normalized.contains("sharing violation") || normalized.contains("resource busy") {
        return CompressionCoreError::FileInUse;
    }

    if normalized.contains("undefined in")
        || normalized.contains("couldn't find trailer dictionary")
        || normalized.contains("catalog dictionary not located")
        || normalized.contains("syntax error")
    {
        return CompressionCoreError::CorruptPdf;
    }

    CompressionCoreError::EngineFailed(stderr.to_string())
}
