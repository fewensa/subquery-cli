/// Subquery config
#[derive(Clone, Debug)]
pub struct Config {
  token: String,
}

impl Config {
  /// Create new config instance
  pub fn new(token: String) -> Self {
    Self { token }
  }
}

impl Config {
  /// Get token
  pub fn token(&self) -> &String {
    &self.token
  }
}
