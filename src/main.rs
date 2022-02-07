use std::process;

use color_eyre::Result;
use structopt::StructOpt;

use crate::command::types::{Opt, SubqueryOpt};
use crate::error::SubqueryError;
use crate::subquery::{Config, Subquery};

mod command;
mod error;
mod initialize;
mod subquery;

#[tokio::main]
async fn main() -> Result<()> {
  initialize::init()?;

  let opt = Opt::from_args();
  if let Err(e) = handle_opt(opt).await {
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

async fn handle_opt(opt: Opt) -> Result<()> {
  let config = Config::new(opt.token);
  let subquery = Subquery::new("https://api.subquery.network", config)?;

  let sopt = opt.command;
  match sopt {
    SubqueryOpt::User { command } => command::handler::handle_user(&subquery, command).await,
    SubqueryOpt::Project { command } => command::handler::handle_project(&subquery, command).await,
    SubqueryOpt::Deployment { command } => {
      command::handler::handle_deployment(&subquery, command).await
    }
    SubqueryOpt::Logs { command } => command::handler::handle_logs(&subquery, command).await,
  }
}
