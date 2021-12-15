use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
  pub avatar_url: String,
  #[serde(rename = "accessToken")]
  pub access_token: String,
  pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
  pub key: String,
  pub name: String,
  #[serde(rename = "avatarUrl")]
  pub avatar_url: String,
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
pub struct Project {
  #[serde(rename = "createdAt")]
  pub created_at: Option<DateTime<Utc>>,
  #[serde(rename = "updatedAt")]
  pub updated_at: Option<DateTime<Utc>>,
  pub key: String,
  pub account: Option<String>,
  pub name: Option<String>,
  pub network: Option<String>,
  pub deployed: Option<bool>,
  #[serde(
    rename = "logoUrl",
    deserialize_with = "crate::subquery::patch::empty_string_as_none"
  )]
  pub logo_url: Option<String>,
  #[serde(deserialize_with = "crate::subquery::patch::empty_string_as_none")]
  pub subtitle: Option<String>,
  #[serde(deserialize_with = "crate::subquery::patch::empty_string_as_none")]
  pub description: Option<String>,
  #[serde(
    rename = "gitRepository",
    deserialize_with = "crate::subquery::patch::empty_string_as_none"
  )]
  pub git_repository: Option<String>,
  pub hide: bool,
  #[serde(
    rename = "dedicateDBKey",
    deserialize_with = "crate::subquery::patch::empty_string_as_none"
  )]
  pub dedicate_db_key: Option<String>,
  #[serde(
    rename = "queryUrl",
    deserialize_with = "crate::subquery::patch::empty_string_as_none"
  )]
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
  #[serde(
    rename = "subFolder",
    deserialize_with = "crate::subquery::patch::empty_string_as_none"
  )]
  pub sub_folder: Option<String>,
  #[serde(deserialize_with = "crate::subquery::patch::empty_string_as_none")]
  pub endpoint: Option<String>,
  #[serde(deserialize_with = "crate::subquery::patch::empty_string_as_none")]
  pub dict_endpoint: Option<String>,
  #[serde(rename = "type")]
  pub type_: DeploymentType,
  #[serde(rename = "queryUrl")]
  pub query_url: String,
  #[serde(rename = "queryClusterUrl")]
  pub query_cluster_url: String,
  pub metadata: DeploymentMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeploymentType {
  #[serde(rename = "primary")]
  Primary,
  #[serde(rename = "stage")]
  Stage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeploymentStatus {
  #[serde(rename = "running")]
  Running,
  #[serde(rename = "error")]
  Error,
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
pub struct Image {
  pub indexer: Vec<String>,
  pub query: Vec<String>,
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
  pub avatar_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeployRequest {
  pub version: String,
  #[serde(rename = "dictEndpoint")]
  pub dict_endpoint: Option<String>,
  #[serde(rename = "indexerImageVersion")]
  pub indexer_image_version: String,
  #[serde(rename = "queryImageVersion")]
  pub query_image_version: String,
  #[serde(rename = "type")]
  pub type_: DeploymentType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncStatus {
  #[serde(rename = "processingBlock")]
  pub processing_block: u32,
  #[serde(rename = "targetBlock")]
  pub target_block: u32,
  #[serde(rename = "totalEntities")]
  pub total_entities: u64,
}
