use thiserror::Error;

#[derive(Error, Debug)]
pub enum Eq2cError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    // #[error("EXR error: {0}")]
    // Exr(#[from] exr::error::Error),
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("Invalid layout dimensions: expected {expected}, got {found}")]
    InvalidDimensions { expected: String, found: String },

    #[error("Tone mapping error: {0}")]
    ToneMapError(String),

    #[error("CLI argument error: {0}")]
    ArgumentError(String),
}

pub type Result<T> = std::result::Result<T, Eq2cError>;
