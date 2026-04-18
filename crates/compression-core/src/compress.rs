use std::{fs, path::{Path, PathBuf}, time::Instant};

use crate::{
    analyze::analyze_pdf,
    engine::CompressionEngine,
    errors::CompressionCoreError,
    models::{CompressionRequest, CompressionResult, CompressionStatus},
    naming::build_collision_safe_output_path,
};
use tempfile::Builder;

const NO_GAIN_BYTES_THRESHOLD: i64 = 32 * 1024;
const NO_GAIN_PERCENT_THRESHOLD: f64 = 1.0;

pub fn compress_pdf(
    request: &CompressionRequest,
    engine: &dyn CompressionEngine,
) -> CompressionResult {
    let started_at = Instant::now();
    let analysis = analyze_pdf(&request.input_path);

    if let Some(error) = analysis.error {
        return CompressionResult {
            status: CompressionStatus::Failed,
            input_path: request.input_path.clone(),
            output_path: None,
            original_bytes: analysis.bytes,
            output_bytes: None,
            reduction_bytes: None,
            reduction_percent: None,
            duration_ms: started_at.elapsed().as_millis(),
            warnings: analysis.warnings,
            error: Some(error),
        };
    }

    let input_path = PathBuf::from(&request.input_path);
    let output_path = build_collision_safe_output_path(
        &input_path,
        request.suffix.as_deref().unwrap_or(".compressed"),
    );

    let temp_artifacts = match create_temp_output_path(&output_path) {
        Ok(artifacts) => artifacts,
        Err(error) => {
            return CompressionResult {
                status: CompressionStatus::Failed,
                input_path: request.input_path.clone(),
                output_path: None,
                original_bytes: analysis.bytes,
                output_bytes: None,
                reduction_bytes: None,
                reduction_percent: None,
                duration_ms: started_at.elapsed().as_millis(),
                warnings: analysis.warnings,
                error: Some(error.into_public()),
            };
        }
    };

    match engine.compress(&input_path, &temp_artifacts.output_path) {
        Ok(output) => {
            let reduction_bytes = analysis.bytes as i64 - output.output_bytes as i64;
            let reduction_percent =
                reduction_bytes as f64 / analysis.bytes.max(1) as f64 * 100.0;

            let status = if reduction_bytes < NO_GAIN_BYTES_THRESHOLD
                || reduction_percent < NO_GAIN_PERCENT_THRESHOLD
            {
                CompressionStatus::NoGain
            } else {
                CompressionStatus::Success
            };

            if status == CompressionStatus::NoGain {
                let _ = fs::remove_file(&temp_artifacts.output_path);

                return CompressionResult {
                    status,
                    input_path: request.input_path.clone(),
                    output_path: None,
                    original_bytes: analysis.bytes,
                    output_bytes: Some(output.output_bytes),
                    reduction_bytes: Some(reduction_bytes),
                    reduction_percent: Some(reduction_percent),
                    duration_ms: started_at.elapsed().as_millis(),
                    warnings: output.warnings,
                    error: None,
                };
            }

            match fs::rename(&temp_artifacts.output_path, &output_path) {
                Ok(()) => CompressionResult {
                    status,
                    input_path: request.input_path.clone(),
                    output_path: Some(output_path.display().to_string()),
                    original_bytes: analysis.bytes,
                    output_bytes: Some(output.output_bytes),
                    reduction_bytes: Some(reduction_bytes),
                    reduction_percent: Some(reduction_percent),
                    duration_ms: started_at.elapsed().as_millis(),
                    warnings: output.warnings,
                    error: None,
                },
                Err(error) => {
                    let _ = fs::remove_file(&temp_artifacts.output_path);
                    CompressionResult {
                        status: CompressionStatus::Failed,
                        input_path: request.input_path.clone(),
                        output_path: None,
                        original_bytes: analysis.bytes,
                        output_bytes: None,
                        reduction_bytes: None,
                        reduction_percent: None,
                        duration_ms: started_at.elapsed().as_millis(),
                        warnings: vec![],
                        error: Some(CompressionCoreError::from(error).into_public()),
                    }
                }
            }
        }
        Err(error) => {
            let _ = fs::remove_file(&temp_artifacts.output_path);
            CompressionResult {
                status: CompressionStatus::Failed,
                input_path: request.input_path.clone(),
                output_path: None,
                original_bytes: analysis.bytes,
                output_bytes: None,
                reduction_bytes: None,
                reduction_percent: None,
                duration_ms: started_at.elapsed().as_millis(),
                warnings: vec![],
                error: Some(error.into_public()),
            }
        }
    }
}

struct TempOutputArtifacts {
    #[allow(dead_code)]
    directory: tempfile::TempDir,
    output_path: PathBuf,
}

fn create_temp_output_path(
    final_output_path: &Path,
) -> Result<TempOutputArtifacts, CompressionCoreError> {
    let output_parent = final_output_path
        .parent()
        .ok_or(CompressionCoreError::WriteDenied)?;
    let output_name = final_output_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("compressed.pdf");

    let directory = Builder::new()
        .prefix(".squeeezo-")
        .tempdir_in(output_parent)
        .map_err(CompressionCoreError::from)?;

    Ok(TempOutputArtifacts {
        output_path: directory.path().join(output_name),
        directory,
    })
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::{
        engine::{CompressionEngine, EngineOutput},
        errors::CompressionCoreError,
        models::{CompressionRequest, CompressionSource, CompressionStatus},
    };

    use super::compress_pdf;

    struct StubEngine {
        output_bytes: u64,
    }

    impl CompressionEngine for StubEngine {
        fn compress(
            &self,
            _input_path: &Path,
            output_path: &Path,
        ) -> Result<EngineOutput, CompressionCoreError> {
            fs::write(output_path, vec![b'x'; self.output_bytes as usize])
                .map_err(CompressionCoreError::from)?;
            Ok(EngineOutput {
                output_bytes: self.output_bytes,
                warnings: vec![],
            })
        }
    }

    fn write_pdf(path: &Path, size: usize) {
        let mut bytes = b"%PDF-1.4\n".to_vec();
        bytes.extend(vec![b'a'; size.saturating_sub(bytes.len())]);
        fs::write(path, bytes).expect("write pdf fixture");
    }

    #[test]
    fn renames_temp_output_into_final_location_on_success() {
        let temp_dir = tempfile::tempdir().expect("temp dir");
        let input_path = temp_dir.path().join("report.pdf");
        write_pdf(&input_path, 100_000);

        let result = compress_pdf(
            &CompressionRequest {
                input_path: input_path.display().to_string(),
                source: CompressionSource::Cli,
                suffix: Some(".compressed".to_string()),
            },
            &StubEngine { output_bytes: 20_000 },
        );

        let expected_output = temp_dir.path().join("report.compressed.pdf");
        assert_eq!(result.status, CompressionStatus::Success);
        assert_eq!(
            result.output_path.as_deref(),
            Some(expected_output.to_string_lossy().as_ref())
        );
        assert!(expected_output.exists());
    }

    #[test]
    fn discards_output_when_compression_has_no_meaningful_gain() {
        let temp_dir = tempfile::tempdir().expect("temp dir");
        let input_path = temp_dir.path().join("report.pdf");
        write_pdf(&input_path, 100_000);

        let result = compress_pdf(
            &CompressionRequest {
                input_path: input_path.display().to_string(),
                source: CompressionSource::Cli,
                suffix: Some(".compressed".to_string()),
            },
            &StubEngine { output_bytes: 99_000 },
        );

        assert_eq!(result.status, CompressionStatus::NoGain);
        assert!(result.output_path.is_none());
        assert!(!temp_dir.path().join("report.compressed.pdf").exists());
    }
}
