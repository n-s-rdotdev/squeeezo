use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::{
    errors::CompressionCoreError,
    models::{AnalyzePdfResult, CompressionWarning},
};

pub fn analyze_pdf(input_path: impl AsRef<Path>) -> AnalyzePdfResult {
    match analyze_pdf_inner(input_path.as_ref()) {
        Ok(result) => result,
        Err(error) => AnalyzePdfResult {
            input_path: input_path.as_ref().display().to_string(),
            bytes: 0,
            is_pdf: false,
            page_count: None,
            warnings: vec![],
            error: Some(error.into_public()),
        },
    }
}

fn analyze_pdf_inner(input_path: &Path) -> Result<AnalyzePdfResult, CompressionCoreError> {
    let resolved = PathBuf::from(input_path);
    if !resolved.exists() {
        return Err(CompressionCoreError::NotFound);
    }

    let metadata = resolved.metadata()?;
    if metadata.len() == 0 {
        return Err(CompressionCoreError::CorruptPdf);
    }

    let mut file = File::open(&resolved)?;
    let mut magic = [0_u8; 5];
    file.read_exact(&mut magic)
        .map_err(|_| CompressionCoreError::CorruptPdf)?;

    let is_pdf = &magic == b"%PDF-";
    if !is_pdf {
        return Err(CompressionCoreError::NotPdf);
    }

    Ok(AnalyzePdfResult {
        input_path: resolved.display().to_string(),
        bytes: metadata.len(),
        is_pdf,
        page_count: None,
        warnings: Vec::<CompressionWarning>::new(),
        error: None,
    })
}
