use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct TokenRotationError(#[from] Box<dyn std::error::Error + Send>);

impl TokenRotationError {
    pub fn new(source: impl std::error::Error + Send + 'static) -> Self {
        Self(Box::new(source))
    }
}
