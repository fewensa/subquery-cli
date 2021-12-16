use structopt::StructOpt;

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
