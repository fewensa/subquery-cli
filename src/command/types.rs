use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames};

use crate::subquery::DeploymentType;

/// Generic command
#[derive(Debug, StructOpt)]
#[structopt(name = "subquery", about = "Subquery CLI")]
pub struct Opt {
  /// Access Token, If not set will read env `SUBQUERY_TOKEN`
  #[structopt(long)]
  pub token: Option<String>,
  /// Subquery opts
  #[structopt(flatten)]
  pub command: SubqueryOpt,
}

#[derive(Debug, StructOpt)]
pub enum SubqueryOpt {
  /// User
  User {
    #[structopt(flatten)]
    command: UserOpt,
  },
  /// Project
  Project {
    #[structopt(flatten)]
    command: ProjectOpt,
  },
  /// Deployment
  Deployment {
    #[structopt(flatten)]
    command: DeploymentOpt,
  },
  /// Query indexer logs
  Logs {
    #[structopt(flatten)]
    command: LogsCommand,
  },
}

#[derive(Debug, StructOpt)]
pub struct LogsCommand {
  /// Org name
  #[structopt(long)]
  pub org: String,
  /// Project key
  #[structopt(long)]
  pub key: String,
  /// Query stage deployment logs
  #[structopt(long)]
  pub stage: bool,
  /// Log level
  #[structopt(long, default_value = "info")]
  pub level: String,
  /// Search keyword
  #[structopt(long)]
  pub keyword: Option<String>,
  /// Rolling query
  #[structopt(long)]
  pub rolling: bool,
  /// Rolling interval seconds, default is 1
  #[structopt(long, default_value = "1")]
  pub interval: u64,
}

#[derive(Debug, StructOpt)]
pub enum DeploymentOpt {
  /// List all deployments
  List {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Project key
    #[structopt(long)]
    key: String,
    /// Output format
    #[structopt(short, long, default_value = "raw")]
    output: OutputFormat,
  },
  /// Deploy
  Deploy {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Project key
    #[structopt(long)]
    key: String,
    /// Output format
    #[structopt(short, long, default_value = "raw")]
    output: OutputFormat,
    /// Command
    #[structopt(flatten)]
    command: DeployCommand,
    /// If the deployment is exists will be replace to new deployment
    #[structopt(long)]
    force: bool,
  },
  /// Delete deployment
  Delete {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Project key
    #[structopt(long)]
    key: String,
    /// Deployment id
    #[structopt(long)]
    id: u64,
  },
  /// Redeploy a deployment
  Redeploy {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Project key
    #[structopt(long)]
    key: String,
    /// Deployment id, type or id you must choose one
    #[structopt(long)]
    id: Option<u64>,
    /// Command
    #[structopt(flatten)]
    command: DeployCommand,
  },
  /// Promote stage to product
  Promote {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Project key
    #[structopt(long)]
    key: String,
    /// Stage deployment id. if not set it will auto detect.
    #[structopt(long)]
    id: Option<u64>,
  },
  /// Query sync status
  SyncStatus {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Project key
    #[structopt(long)]
    key: String,
    /// Stage deployment id
    #[structopt(long)]
    id: u64,
    /// Rolling query
    #[structopt(long)]
    rolling: bool,
    /// Rolling interval seconds, default is 1
    #[structopt(long, default_value = "1")]
    interval: u64,
  },
}

#[derive(Debug, StructOpt)]
pub struct DeployCommand {
  /// Which branch of git repository
  #[structopt(long)]
  pub branch: String,
  /// The commit of branch, default is latest commit id
  #[structopt(long)]
  pub commit: Option<String>,
  /// Override Network endpoint
  #[structopt(long)]
  pub endpoint: Option<String>,
  /// Override Dictionary endpoint
  #[structopt(long)]
  pub dict_endpoint: Option<String>,
  /// Indexer Version (@subql/node)
  #[structopt(long)]
  pub indexer_image_version: Option<String>,
  /// Query Version (@subql/query)
  #[structopt(long)]
  pub query_image_version: Option<String>,
  /// Deployment type [stage, primary]
  #[structopt(long = "type", default_value = "stage")]
  pub type_: DeploymentType,
  /// Sub folder
  #[structopt(long)]
  pub sub_folder: Option<String>,
  /// Batch size for indexer
  #[structopt(long, default_value = "30")]
  pub indexer_batch_size: u32,
}

#[derive(Debug, StructOpt)]
pub enum UserOpt {
  /// User info
  Info,
  /// Show all organizations
  Orgs,
}

#[derive(Debug, StructOpt)]
pub enum ProjectOpt {
  /// Create a project
  Create {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Project key
    #[structopt(long)]
    key: String,
    /// Project name, if not set it, default use key
    #[structopt(long)]
    name: Option<String>,
    /// Subtitle
    #[structopt(long)]
    subtitle: Option<String>,
    /// Description
    #[structopt(long)]
    description: Option<String>,
    /// Github repository url
    #[structopt(long)]
    repo: String,
    /// Hide project in explorer, default is true
    #[structopt(long)]
    hide: Option<bool>,
    /// Check if the project not exists then create this
    #[structopt(long)]
    check: bool,
  },
  /// Update a project
  Update {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Project key
    #[structopt(long)]
    key: String,
    /// Project name
    #[structopt(long)]
    name: Option<String>,
    /// Subtitle
    #[structopt(long)]
    subtitle: Option<String>,
    /// Description
    #[structopt(long)]
    description: Option<String>,
    /// Hide project in explorer
    #[structopt(long)]
    hide: Option<bool>,
  },
  /// Delete a project
  Delete {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Project name
    #[structopt(long)]
    key: String,
  },
  /// Show all projects
  List {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Output format
    #[structopt(short, long, default_value = "raw")]
    output: OutputFormat,
  },
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum OutputFormat {
  Json,
  Raw,
  Table,
}
