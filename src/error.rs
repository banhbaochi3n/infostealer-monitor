use grammers_client::client::bots::{AuthorizationError, InvocationError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot load .env file: {0}")]
    EnvError(#[from] dotenvy::Error),
    #[error("IO error, cannot read session file or smth idk: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Cannot authorize: {0}")]
    AuthError(#[from] AuthorizationError),
    #[error("Invocation Error: {0}")]
    InvocationError(#[from] InvocationError),
}
