use thiserror::Error;
use std::io;

#[derive(Error,Debug)]
pub enum SerdeFileError
{
    #[error("io error")]
    IoErr(#[from] io::Error),

    #[error("serde error")]
    SerdeError(#[from] serde_yaml::Error)
}