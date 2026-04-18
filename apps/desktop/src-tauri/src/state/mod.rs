use std::{
    fs,
    path::PathBuf,
    sync::RwLock,
};

use compression_core::{AppSettings, PartialAppSettings, RecentJobRecord};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistentState {
    settings: AppSettings,
    recent_jobs: Vec<RecentJobRecord>,
}

impl Default for PersistentState {
    fn default() -> Self {
        Self {
            settings: AppSettings::default(),
            recent_jobs: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct PersistentStateHandle {
    path: PathBuf,
    state: RwLock<PersistentState>,
}

impl PersistentStateHandle {
    pub fn load(app: &AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let app_dir = app.path().app_local_data_dir()?;
        fs::create_dir_all(&app_dir)?;

        let path = app_dir.join("squeeezo-state.json");
        let state = fs::read_to_string(&path)
            .ok()
            .and_then(|contents| serde_json::from_str::<PersistentState>(&contents).ok())
            .unwrap_or_default();

        Ok(Self {
            path,
            state: RwLock::new(state),
        })
    }

    pub fn settings(&self) -> AppSettings {
        self.state
            .read()
            .expect("persistent state read")
            .settings
            .clone()
    }

    pub fn recent_jobs(&self) -> Vec<RecentJobRecord> {
        self.state
            .read()
            .expect("persistent state read")
            .recent_jobs
            .clone()
    }

    pub fn clear_recent_jobs(&self) -> Result<(), String> {
        {
            let mut state = self.state.write().map_err(|_| "State lock poisoned.")?;
            state.recent_jobs.clear();
        }
        self.persist()
    }

    pub fn update_settings(
        &self,
        partial: PartialAppSettings,
    ) -> Result<AppSettings, String> {
        let next_settings = {
            let mut state = self.state.write().map_err(|_| "State lock poisoned.")?;
            if let Some(output_suffix) = partial.output_suffix {
                state.settings.output_suffix = output_suffix;
            }
            if let Some(keep_recent_jobs) = partial.keep_recent_jobs {
                state.settings.keep_recent_jobs = keep_recent_jobs;
                state.recent_jobs.truncate(keep_recent_jobs);
            }
            if let Some(reveal_output_on_success) = partial.reveal_output_on_success {
                state.settings.reveal_output_on_success = reveal_output_on_success;
            }
            if let Some(open_output_on_success) = partial.open_output_on_success {
                state.settings.open_output_on_success = open_output_on_success;
            }
            state.settings.clone()
        };

        self.persist()?;
        Ok(next_settings)
    }

    pub fn push_recent_job(&self, job: RecentJobRecord) -> Result<(), String> {
        {
            let mut state = self.state.write().map_err(|_| "State lock poisoned.")?;
            let keep_recent_jobs = state.settings.keep_recent_jobs.max(1);
            state.recent_jobs.insert(0, job);
            state.recent_jobs.truncate(keep_recent_jobs);
        }

        self.persist()
    }

    fn persist(&self) -> Result<(), String> {
        let state = self
            .state
            .read()
            .map_err(|_| "State lock poisoned.")?;
        let contents =
            serde_json::to_string_pretty(&*state).map_err(|error| error.to_string())?;
        fs::write(&self.path, contents).map_err(|error| error.to_string())
    }
}
