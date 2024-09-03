use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{api_name} is not implemented")]
    Unimplemented { api_name: String, version: String },
    #[error("{0} not found")]
    NotFound(String),
}
