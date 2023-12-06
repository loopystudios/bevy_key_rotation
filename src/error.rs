use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct TokenRotationError(#[from] Box<dyn std::error::Error + Send + Sync>);

impl TokenRotationError {
    pub fn new(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self(Box::new(source))
    }
}
