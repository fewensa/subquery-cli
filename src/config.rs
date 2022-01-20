use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::subquery::User;

/// Get subquery home path
pub fn subquery_home() -> PathBuf {
  let path_env = std::env::var("SUBQUERY_HOME");
  let is_from_env = path_env.is_ok();
  let basic_path = path_env
    .map(|v| Path::new(&v).join(""))
    .ok()
    .or_else(dirs::home_dir)
    .or_else(|| std::env::current_exe().ok())
    .unwrap_or_else(std::env::temp_dir);
  let mut base_path = basic_path;
  if !is_from_env {
    base_path = base_path.join(".subquery");
  }
  base_path
}

#[derive(Clone, Debug)]
pub struct Config {
  base_path: PathBuf,
}

impl Config {
  pub fn new(base_path: PathBuf) -> Self {
    Self { base_path }
  }
}

impl Default for Config {
  fn default() -> Self {
    Self::new(subquery_home())
  }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct ConfigStruct {
  user: Option<User>,
}

impl Config {
  fn config_file(&self) -> PathBuf {
    self.base_path.join("subquery.json")
  }
  fn write(&self, cfg: &ConfigStruct) -> color_eyre::Result<()> {
    if !self.base_path.exists() {
      std::fs::create_dir_all(&self.base_path)?;
    }
    let path = self.config_file();
    // tracing::trace!("Write config to {:?}", path);
    let json = serde_json::to_string_pretty(cfg)?;
    if let Some(parent) = path.parent() {
      if !parent.exists() {
        std::fs::create_dir_all(parent)?;
      }
    }
    std::fs::write(path, json)?;
    Ok(())
  }
  fn read(&self) -> color_eyre::Result<Option<ConfigStruct>> {
    let path = self.config_file();
    // tracing::trace!("Read config from {:?}", path);
    if !path.exists() {
      return Ok(None);
    }
    let json = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
  }
}

impl Config {
  fn restore(&self) -> color_eyre::Result<Option<ConfigStruct>> {
    self.read()
  }

  pub fn restore_user(&self) -> color_eyre::Result<Option<User>> {
    match self.restore()? {
      Some(v) => Ok(v.user),
      None => Ok(None),
    }
  }
}

impl Config {
  pub fn store_user(&self, user: User) -> color_eyre::Result<()> {
    let mut cfg = self.restore()?.unwrap_or_default();
    cfg.user = Some(user);
    self.write(&cfg)
  }
}
