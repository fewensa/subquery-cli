use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
#[allow(dead_code)]
pub enum SubqueryError {
  #[error("Io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Custom error: {0}")]
  Custom(String),

  #[error("Wrap error: {0}")]
  Wrap(Box<dyn std::error::Error + Send + Sync>),
}
