//! PpmError uses the `thiserror` crate to define error types that can be returned
//! by this library. 

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PpmError {
    #[error("no valid file specified")]
    InvalidFile,

    #[error("file i/o error: context: {0}; file = {1}, error = {2}")]
    FileIO(String, String, String),
    
    #[error("image data not set")]
    InvalidImageData,
}

