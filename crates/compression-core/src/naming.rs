use std::path::{Path, PathBuf};

pub fn build_output_path(input_path: &Path, suffix: &str) -> PathBuf {
    let parent = input_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_default();
    let stem = input_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("document");
    let extension = input_path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("pdf");

    parent.join(format!("{stem}{suffix}.{extension}"))
}

pub fn build_collision_safe_output_path(input_path: &Path, suffix: &str) -> PathBuf {
    let candidate = build_output_path(input_path, suffix);
    if !candidate.exists() {
        return candidate;
    }

    let parent = candidate
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_default();
    let stem = candidate
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("document");
    let extension = candidate
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("pdf");

    for index in 2.. {
        let collision_path = parent.join(format!("{stem}-{index}.{extension}"));
        if !collision_path.exists() {
            return collision_path;
        }
    }

    unreachable!("collision-safe path generation exhausted unexpectedly")
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use super::{build_collision_safe_output_path, build_output_path};

    #[test]
    fn appends_default_suffix_to_pdf() {
        let output = build_output_path(
            PathBuf::from("/tmp/report.pdf").as_path(),
            ".compressed",
        );

        assert_eq!(output, PathBuf::from("/tmp/report.compressed.pdf"));
    }

    #[test]
    fn increments_collision_suffixes() {
        let temp_dir = tempfile::tempdir().expect("temp dir");
        let input = temp_dir.path().join("report.pdf");
        let collided = temp_dir.path().join("report.compressed.pdf");

        File::create(&collided).expect("collided file");

        let output = build_collision_safe_output_path(&input, ".compressed");

        assert_eq!(output, temp_dir.path().join("report.compressed-2.pdf"));
    }
}
