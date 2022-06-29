use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::string_empty_as_none;
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  #[serde(rename = "updatedAt")]
  pub updated_at: Option<DateTime<Utc>>,
  pub id: String,
  pub email: String,
  pub username: String,
  #[serde(rename = "displayName")]
  pub display_name: String,
  #[serde(rename = "avatarUrl")]
  pub avatar_url: Option<String>,
  #[serde(rename = "existsToken")]
  pub exists_token: bool,
  pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
  pub key: String,
  pub name: String,
  #[serde(rename = "avatarUrl")]
  pub avatar_url: Option<String>,
  #[serde(rename = "type")]
  pub type_: AccountType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AccountType {
  #[serde(rename = "user")]
  User,
  #[serde(rename = "org")]
  Org,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiVersion {
  #[serde(rename = "v2")]
  Latest,
  #[serde(rename = "v1")]
  V1,
  #[serde(rename = "v2")]
  V2,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
  #[serde(rename = "apiVersion")]
  pub api_version: ApiVersion,
  #[serde(rename = "createdAt")]
  pub created_at: Option<DateTime<Utc>>,
  #[serde(rename = "updatedAt")]
  pub updated_at: Option<DateTime<Utc>>,
  pub key: String,
  pub account: Option<String>,
  pub name: Option<String>,
  pub network: Option<String>,
  pub deployed: Option<bool>,
  #[serde(rename = "logoUrl", with = "string_empty_as_none")]
  pub logo_url: Option<String>,
  #[serde(with = "string_empty_as_none")]
  pub subtitle: Option<String>,
  #[serde(with = "string_empty_as_none")]
  pub description: Option<String>,
  #[serde(rename = "gitRepository", with = "string_empty_as_none")]
  pub git_repository: Option<String>,
  pub hide: Option<bool>,
  #[serde(rename = "dedicateDBKey")]
  pub dedicate_db_key: Option<String>,
  #[serde(rename = "queryUrl", with = "string_empty_as_none")]
  pub query_url: Option<String>,
  pub deployment: Option<Deployment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateProjectResponse {
  pub key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deployment {
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  #[serde(rename = "updatedAt")]
  pub updated_at: Option<DateTime<Utc>>,
  pub id: u64,
  #[serde(rename = "projectKey")]
  pub project_key: String,
  pub version: String,
  pub status: DeploymentStatus,
  pub cluster: String,
  #[serde(rename = "indexerImage")]
  pub indexer_image: String,
  #[serde(rename = "queryImage")]
  pub query_image: String,
  #[serde(rename = "subFolder", with = "string_empty_as_none")]
  pub sub_folder: Option<String>,
  #[serde(with = "string_empty_as_none")]
  pub endpoint: Option<String>,
  #[serde(rename = "dictEndpoint", with = "string_empty_as_none")]
  pub dict_endpoint: Option<String>,
  #[serde(rename = "type")]
  pub type_: DeploymentType,
  #[serde(rename = "queryUrl")]
  pub query_url: String,
  #[serde(rename = "queryClusterUrl")]
  pub query_cluster_url: Option<String>,
  pub metadata: Option<DeploymentMetadata>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDeployRequest {
  /// The commit of branch, default is latest commit id
  #[serde(rename = "version")]
  pub commit: Option<String>,
  /// Override Network endpoint
  #[serde(with = "string_empty_as_none")]
  pub endpoint: Option<String>,
  /// Override Dictionary endpoint
  #[serde(rename = "dictEndpoint", with = "string_empty_as_none")]
  pub dict_endpoint: Option<String>,
  /// Indexer Version (@subql/node)
  #[serde(rename = "indexerImageVersion")]
  pub indexer_image_version: Option<String>,
  /// Query Version (@subql/query)
  #[serde(rename = "queryImageVersion")]
  pub query_image_version: Option<String>,
  /// Deployment type
  #[serde(rename = "type")]
  pub type_: DeploymentType,
  #[serde(rename = "subFolder", with = "string_empty_as_none")]
  pub sub_folder: Option<String>,
  #[serde(rename = "advancedSettings")]
  pub advanced_settings: AdvancedSettings,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum DeploymentType {
  #[serde(rename = "primary")]
  Primary,
  #[serde(rename = "stage")]
  Stage,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DeploymentStatus {
  #[serde(rename = "processing")]
  Processing,
  #[serde(rename = "running")]
  Running,
  #[serde(rename = "error")]
  Error,
  #[serde(rename = "stopped")]
  Stopped,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentMetadata {
  pub role: String,
  #[serde(rename = "isSample")]
  pub is_sample: bool,
  #[serde(rename = "enableTimestamp")]
  pub enable_timestamp: bool,
  #[serde(rename = "indexerBatchSize")]
  pub indexer_batch_size: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Branch {
  pub name: String,
  pub protected: bool,
  pub commit: BranchCommit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BranchCommit {
  pub sha: String,
  pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commit {
  pub sha: String,
  pub message: String,
  pub time: DateTime<Utc>,
  pub author: CommitAuthor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommitAuthor {
  pub name: String,
  #[serde(rename = "avatarUrl")]
  pub avatar_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeployRequest {
  pub version: String,
  pub endpoint: Option<String>,
  #[serde(rename = "dictEndpoint")]
  pub dict_endpoint: Option<String>,
  #[serde(rename = "indexerImageVersion")]
  pub indexer_image_version: String,
  #[serde(rename = "queryImageVersion")]
  pub query_image_version: String,
  #[serde(rename = "type")]
  pub type_: DeploymentType,
  #[serde(rename = "advancedSettings")]
  pub advanced: AdvancedSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncStatus {
  #[serde(rename = "processingBlock")]
  pub processing_block: u32,
  #[serde(rename = "targetBlock")]
  pub target_block: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Log {
  #[serde(rename = "startTime")]
  pub start_time: DateTime<Utc>,
  #[serde(rename = "endTime")]
  pub end_time: DateTime<Utc>,
  #[serde(rename = "searchAfterId")]
  pub search_after_id: Vec<u64>,
  pub result: Vec<LogResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogResult {
  pub level: String,
  pub message: String,
  pub category: String,
  pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubIndexerSettings {
  #[serde(rename = "batchSize")]
  pub batch_size: u32,
  pub subscription: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubQuerySettings {
  pub subscription: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdvancedSettings {
  #[serde(rename = "@subql/node")]
  pub subql_node: SubIndexerSettings,
  #[serde(rename = "@subql/query")]
  pub subql_query: SubQuerySettings,
}

impl AdvancedSettings {
  pub fn new(batch_size: u32, subscription: bool) -> Self {
    Self {
      subql_node: SubIndexerSettings {
        batch_size,
        subscription,
      },
      subql_query: SubQuerySettings { subscription },
    }
  }
}
