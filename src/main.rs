use color_eyre::Result;
use structopt::StructOpt;

use crate::command::types::Opt;
use crate::config::Config;
use crate::subquery::Subquery;

mod command;
mod config;
mod error;
mod initialize;
mod subquery;

#[tokio::main]
async fn main() -> Result<()> {
  initialize::init()?;

  let config = Config::default();
  let subquery = Subquery::new("https://api.subquery.network", config)?;

  let opt = Opt::from_args();
  match opt {
    Opt::Login { sid } => command::handler::handle_login(&subquery, sid).await?,
  }

  Ok(())
}
