use crate::subquery::DeploymentType;
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames};

#[derive(Debug, StructOpt)]
#[structopt(name = "subquery", about = "Subquery CLI")]
pub enum Opt {
  /// Login subquery
  Login {
    /// Connect sid
    #[structopt(long)]
    sid: String,
  },
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
  Redeploy {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Project key
    #[structopt(long)]
    key: String,
    /// Deployment id
    #[structopt(long)]
    id: u64,
    /// Command
    #[structopt(flatten)]
    command: DeployCommand,
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
  /// Deployment type
  #[structopt(long, default_value = "stage")]
  pub type_: DeploymentType,
  /// Sub folder
  #[structopt(long)]
  pub sub_folder: Option<String>,
}

#[derive(Debug, StructOpt)]
pub enum UserOpt {
  /// User info
  Info,
  /// Show current access token
  AccessToken,
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
  },
  /// Update a project
  Update {
    /// Org name
    #[structopt(long)]
    org: String,
    /// Project name
    #[structopt(long)]
    key: String,
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
    name: String,
  },
  /// Show all projects
  List {
    /// Org name
    #[structopt(long)]
    org: String,
  },
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum OutputFormat {
  Json,
  Raw,
  Table,
}
