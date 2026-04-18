use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CompressionSource {
    Desktop,
    FinderAction,
    Cli,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompressionWarning {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompressionError {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzePdfResult {
    pub input_path: String,
    pub bytes: u64,
    pub is_pdf: bool,
    pub page_count: Option<u32>,
    pub warnings: Vec<CompressionWarning>,
    pub error: Option<CompressionError>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompressionRequest {
    pub input_path: String,
    pub source: CompressionSource,
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompressionResult {
    pub status: CompressionStatus,
    pub input_path: String,
    pub output_path: Option<String>,
    pub original_bytes: u64,
    pub output_bytes: Option<u64>,
    pub reduction_bytes: Option<i64>,
    pub reduction_percent: Option<f64>,
    pub duration_ms: u128,
    pub warnings: Vec<CompressionWarning>,
    pub error: Option<CompressionError>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CompressionStatus {
    Success,
    NoGain,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub output_suffix: String,
    pub keep_recent_jobs: usize,
    pub reveal_output_on_success: bool,
    pub open_output_on_success: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            output_suffix: ".compressed".to_string(),
            keep_recent_jobs: 20,
            reveal_output_on_success: false,
            open_output_on_success: false,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PartialAppSettings {
    pub output_suffix: Option<String>,
    pub keep_recent_jobs: Option<usize>,
    pub reveal_output_on_success: Option<bool>,
    pub open_output_on_success: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecentJobRecord {
    pub id: String,
    pub created_at: String,
    pub source: RecentJobSource,
    pub input_path: String,
    pub output_path: Option<String>,
    pub original_bytes: u64,
    pub output_bytes: Option<u64>,
    pub status: CompressionStatus,
    pub error_code: Option<String>,
    pub duration_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RecentJobSource {
    Desktop,
    FinderAction,
}
