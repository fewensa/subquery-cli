use std::process;

use color_eyre::Result;
use structopt::StructOpt;

use crate::command::types::Opt;
use crate::config::Config;
use crate::error::SubqueryError;
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
  if let Err(e) = handle_opt(opt, &subquery).await {
    match e.downcast_ref::<SubqueryError>() {
      Some(SubqueryError::Api(api, code, message)) => {
        eprintln!("Failed to request: [{}] [{}]: {}", api, code, message);
        process::exit(1);
      }
      Some(SubqueryError::Custom(msg)) => {
        eprintln!("Custom: {}", msg);
        process::exit(1);
      }
      _ => {}
    }
    return Err(e);
  }
  Ok(())
}

async fn handle_opt(opt: Opt, subquery: &Subquery) -> Result<()> {
  match &opt {
    Opt::Login { .. } => {}
    _ => {
      if !subquery.is_login()? {
        eprintln!("Please run login first");
        process::exit(exitcode::CONFIG);
      }
    }
  }
  match opt {
    Opt::Login { sid } => command::handler::handle_login(subquery, sid).await,
    Opt::User { command } => command::handler::handle_user(subquery, command).await,
    Opt::Project { command } => command::handler::handle_project(subquery, command).await,
    Opt::Deployment { command } => command::handler::handle_deployment(subquery, command).await,
    Opt::Logs { command } => command::handler::handle_logs(subquery, command).await,
  }
}
