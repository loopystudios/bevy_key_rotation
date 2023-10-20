use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct TokenRotationError(#[from] Box<dyn std::error::Error + Send>);
