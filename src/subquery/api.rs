#![allow(dead_code)]

use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::error::SubqueryError;
use crate::subquery::{
  Branch, Commit, CreateProjectResponse, DeployRequest, Deployment, Image, Log, Project,
  SyncStatus, User,
};
use crate::Config;

#[derive(Clone, Debug)]
pub struct Subquery {
  client: Client,
  endpoint: String,
  config: Config,
}

impl Subquery {
  pub fn new(endpoint: impl AsRef<str>, config: Config) -> color_eyre::Result<Self> {
    let client = Client::builder()
      .timeout(std::time::Duration::from_secs(10))
      .build()?;
    Ok(Self {
      client,
      endpoint: endpoint.as_ref().to_string(),
      config,
    })
  }
}

impl Subquery {
  pub fn config(&self) -> &Config {
    &self.config
  }

  pub fn is_login(&self) -> color_eyre::Result<bool> {
    let saved_user = self.config().restore_user()?;
    Ok(saved_user.is_some())
  }
}

impl Subquery {
  fn api(&self, api: impl AsRef<str>) -> String {
    format!("{}{}", self.endpoint, api.as_ref())
  }

  fn request(&self, method: Method, api: impl AsRef<str>) -> color_eyre::Result<RequestBuilder> {
    let api = self.api(api);
    let mut builder = self.client.request(method, &api);
    let saved_user = self.config().restore_user()?;
    if let Some(u) = saved_user {
      if &api[..] != "/user" {
        builder = builder.bearer_auth(u.access_token.clone());
      }
    }
    Ok(builder)
  }

  fn deserialize<T: DeserializeOwned>(&self, json: impl AsRef<str>) -> color_eyre::Result<T> {
    let json = json.as_ref();
    if json.starts_with("{") {
      let value: serde_json::Value = serde_json::from_str(json)?;
      if let Some(sc) = value.get("statusCode") {
        return Err(
          SubqueryError::Api(
            sc.as_u64().unwrap_or(u64::MAX),
            value
              .get("message")
              .map(|v| v.as_str().unwrap_or("Unknown error").to_string())
              .unwrap_or("No message from server".to_string()),
          )
          .into(),
        );
      }
    }
    Ok(serde_json::from_str(json)?)
  }
}

impl Subquery {
  pub async fn user(&self, sid: impl AsRef<str>) -> color_eyre::Result<User> {
    let response = self
      .request(Method::GET, "/user")?
      .header("Cookie", format!("connect.sid={}", sid.as_ref()))
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(response)
  }

  pub async fn create_project(
    &self,
    project: Project,
  ) -> color_eyre::Result<CreateProjectResponse> {
    let response = self
      .request(Method::POST, "/subqueries")?
      .json(&project)
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(response)
  }

  pub async fn update_project(&self, project: Project) -> color_eyre::Result<()> {
    let mut data = HashMap::new();
    if let Some(v) = project.name {
      data.insert("name", serde_json::Value::String(v));
    }
    if let Some(v) = project.subtitle {
      data.insert("subtitle", serde_json::Value::String(v));
    }
    if let Some(v) = project.description {
      data.insert("description", serde_json::Value::String(v));
    }
    if let Some(v) = project.hide {
      data.insert("hide", serde_json::Value::Bool(v));
    }
    let _response = self
      .request(Method::PUT, format!("/subqueries/{}", project.key))?
      .json(&data)
      .send()
      .await?
      .text()
      .await?;
    Ok(())
  }

  pub async fn delete_project(&self, key: impl AsRef<str>) -> color_eyre::Result<()> {
    let _response = self
      .request(Method::DELETE, format!("/subqueries/{}", key.as_ref()))?
      .send()
      .await?
      .text()
      .await?;
    Ok(())
  }

  pub async fn projects(&self, account: String) -> color_eyre::Result<Vec<Project>> {
    // https://api.subquery.network/user/projects?account=fewensa
    let response = self
      .request(Method::GET, format!("/user/projects?account={}", account))?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(response)
  }

  pub async fn project(&self, key: impl AsRef<str>) -> color_eyre::Result<Option<Project>> {
    // https://api.subquery.network/subqueries/fewensa/pangolin-test
    let response = self
      .request(Method::GET, format!("/subqueries/{}", key.as_ref()))?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(response)
  }

  pub async fn deployments(&self, key: impl AsRef<str>) -> color_eyre::Result<Vec<Deployment>> {
    let response = self
      .request(
        Method::GET,
        format!("/subqueries/{}/deployments", key.as_ref()),
      )?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(response)
  }

  pub async fn branches(&self, key: impl AsRef<str>) -> color_eyre::Result<Vec<Branch>> {
    let response = self
      .request(
        Method::GET,
        format!("/info/github/{}/branches", key.as_ref()),
      )?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(response)
  }

  pub async fn image(&self) -> color_eyre::Result<Image> {
    let response = self
      .request(Method::GET, "/info/images")?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(response)
  }

  pub async fn commit(
    &self,
    key: impl AsRef<str>,
    branch: impl AsRef<str>,
  ) -> color_eyre::Result<Vec<Commit>> {
    let response = self
      .request(
        Method::GET,
        format!("/info/github/{}/commits/{}", key.as_ref(), branch.as_ref()),
      )?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(response)
  }

  pub async fn deploy(
    &self,
    key: impl AsRef<str>,
    data: &DeployRequest,
  ) -> color_eyre::Result<Project> {
    let response = self
      .request(
        Method::POST,
        format!("/subqueries/{}/deployments", key.as_ref()),
      )?
      .json(data)
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(response)
  }

  pub async fn redeploy(
    &self,
    key: impl AsRef<str>,
    id: u64,
    data: &DeployRequest,
  ) -> color_eyre::Result<()> {
    let _response = self
      .request(
        Method::PUT,
        format!("/subqueries/{}/deployments/{}", key.as_ref(), id),
      )?
      .json(data)
      .send()
      .await?
      .text()
      .await?;
    Ok(())
  }

  pub async fn delete_deploy(&self, key: impl AsRef<str>, id: u64) -> color_eyre::Result<()> {
    let _response = self
      .request(
        Method::DELETE,
        format!("/subqueries/{}/deployments/{}", key.as_ref(), id),
      )?
      .send()
      .await?
      .text()
      .await?;
    Ok(())
  }

  pub async fn rebase_deployment(&self, key: impl AsRef<str>, id: u64) -> color_eyre::Result<()> {
    let _response = self
      .request(
        Method::POST,
        format!("/subqueries/{}/deployments/{}/release", key.as_ref(), id),
      )?
      .send()
      .await?
      .text()
      .await?;
    Ok(())
  }

  pub async fn deployment_sync_status(
    &self,
    key: impl AsRef<str>,
    id: u64,
  ) -> color_eyre::Result<SyncStatus> {
    let response = self
      .request(
        Method::GET,
        format!(
          "/subqueries/{}/deployments/{}/sync-status",
          key.as_ref(),
          id
        ),
      )?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(response)
  }

  pub async fn logs(
    &self,
    key: impl AsRef<str>,
    stage: bool,
    level: impl AsRef<str>,
  ) -> color_eyre::Result<Log> {
    self.search_logs(key, stage, level, None).await
  }

  pub async fn search_logs(
    &self,
    key: impl AsRef<str>,
    stage: bool,
    level: impl AsRef<str>,
    keyword: Option<String>,
  ) -> color_eyre::Result<Log> {
    let response = self
      .request(
        Method::GET,
        format!(
          "/subqueries/{}/logs?stage={}&level={}&keyword={}",
          key.as_ref(),
          stage,
          level.as_ref(),
          keyword.unwrap_or(Default::default())
        ),
      )?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(response)
  }
}
