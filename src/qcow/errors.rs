use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    #[error("qcow file not found")]
    FileNotFound(std::io::Error),
    #[error("qcow file failed to parse")]
    ParseError(#[from] binread::Error),
}