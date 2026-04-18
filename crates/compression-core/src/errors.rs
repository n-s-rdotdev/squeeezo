use std::io;
use thiserror::Error;

use crate::models::CompressionError;

#[derive(Debug, Error)]
pub enum CompressionCoreError {
    #[error("File not found.")]
    NotFound,
    #[error("Input is not a PDF.")]
    NotPdf,
    #[error("Input PDF is corrupt or empty.")]
    CorruptPdf,
    #[error("Input PDF requires a password.")]
    PasswordProtected,
    #[error("Compression engine is not available yet.")]
    EngineMissing,
    #[error("Compression engine failed.")]
    EngineFailed(String),
    #[error("Unable to read input.")]
    ReadDenied,
    #[error("Unable to write output.")]
    WriteDenied,
    #[error("Insufficient disk space for output.")]
    OutOfSpace,
    #[error("The file is currently locked by another application.")]
    FileInUse,
    #[error("Ghostscript is not supported on this platform.")]
    UnsupportedPlatform,
    #[error("Unknown failure.")]
    Unknown,
}

impl CompressionCoreError {
    pub fn into_public(self) -> CompressionError {
        match self {
            Self::NotFound => CompressionError {
                code: "NOT_FOUND".to_string(),
                message: "The selected file no longer exists.".to_string(),
                details: None,
            },
            Self::NotPdf => CompressionError {
                code: "NOT_PDF".to_string(),
                message: "The selected file is not a valid PDF.".to_string(),
                details: None,
            },
            Self::CorruptPdf => CompressionError {
                code: "CORRUPT_PDF".to_string(),
                message: "The selected PDF is empty or malformed.".to_string(),
                details: None,
            },
            Self::PasswordProtected => CompressionError {
                code: "PASSWORD_PROTECTED".to_string(),
                message: "The selected PDF is password protected.".to_string(),
                details: None,
            },
            Self::EngineMissing => CompressionError {
                code: "ENGINE_MISSING".to_string(),
                message: "Ghostscript is not available on this system.".to_string(),
                details: None,
            },
            Self::EngineFailed(stderr) => CompressionError {
                code: "ENGINE_FAILED".to_string(),
                message: "The compression engine exited with an error."
                    .to_string(),
                details: Some(stderr),
            },
            Self::ReadDenied => CompressionError {
                code: "READ_DENIED".to_string(),
                message: "Squeeezo could not read the selected file."
                    .to_string(),
                details: None,
            },
            Self::WriteDenied => CompressionError {
                code: "WRITE_DENIED".to_string(),
                message: "Squeeezo could not create the output file."
                    .to_string(),
                details: None,
            },
            Self::OutOfSpace => CompressionError {
                code: "OUT_OF_SPACE".to_string(),
                message: "There is not enough disk space to write the output."
                    .to_string(),
                details: None,
            },
            Self::FileInUse => CompressionError {
                code: "FILE_IN_USE".to_string(),
                message: "The PDF or its destination is currently in use."
                    .to_string(),
                details: None,
            },
            Self::UnsupportedPlatform => CompressionError {
                code: "UNSUPPORTED_PLATFORM".to_string(),
                message: "Ghostscript is not configured for this platform."
                    .to_string(),
                details: None,
            },
            Self::Unknown => CompressionError {
                code: "UNKNOWN".to_string(),
                message: "An unknown error occurred.".to_string(),
                details: None,
            },
        }
    }
}

impl From<io::Error> for CompressionCoreError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => Self::NotFound,
            io::ErrorKind::PermissionDenied => Self::ReadDenied,
            io::ErrorKind::StorageFull => Self::OutOfSpace,
            io::ErrorKind::WouldBlock => Self::FileInUse,
            _ => Self::Unknown,
        }
    }
}
