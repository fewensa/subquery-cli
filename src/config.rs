use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::subquery::User;

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
    let tmp_path = std::env::temp_dir().join("subquery");
    let base_path = match std::env::current_exe() {
      Ok(v) => match v.parent() {
        Some(p) => p.join(""),
        None => tmp_path,
      },
      Err(_) => tmp_path,
    };
    Self::new(base_path)
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
