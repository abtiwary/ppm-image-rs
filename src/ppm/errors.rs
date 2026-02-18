use thiserror::Error;

#[derive(Error, Debug)]
pub enum PpmError {
    #[error("no valid file specified")]
    InvalidFile,

    #[error("file i/o error: could not read {0}, error = {1}")]
    FileIO(String, String),
    
    #[error("image data not set")]
    InvalidImageData,
}

