use thiserror::Error;

#[derive(Error, Debug)]
pub enum CopulaError {
    #[error("Invalid parameter: {message}")]
    InvalidParameter { message: String },
    
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    
    #[error("Data contains invalid values: {message}")]
    InvalidData { message: String },
    
    #[error("Estimation failed: {message}")]
    EstimationFailed { message: String },
    
    #[error("Mathematical error: {message}")]
    MathError { message: String },
    
    #[error("Not implemented: {feature}")]
    NotImplemented { feature: String },
}

pub type Result<T> = std::result::Result<T, CopulaError>;
