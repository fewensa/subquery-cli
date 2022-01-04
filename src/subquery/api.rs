#![allow(dead_code)]

use std::collections::HashMap;

use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::error::SubqueryError;
use crate::subquery::{
  Branch, Commit, CreateDeployRequest, CreateProjectResponse, Deployment, Image, Log, Project,
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
        builder = builder.bearer_auth(u.access_token);
      }
    }
    Ok(builder)
  }

  fn parse_github_repo(&self, repo_url: impl AsRef<str>) -> color_eyre::Result<String> {
    let url = repo_url.as_ref();
    let parts0: Vec<&str> = url.split('?').collect();
    let first = parts0
      .get(0)
      .ok_or_else(|| SubqueryError::Custom(format!("Wrong repository url: {}", url)))?;
    let repo_url = first.replace(".git", "");
    let parts1: Vec<&str> = repo_url.split('/').collect();
    let len = parts1.len();
    let org = parts1
      .get(len - 2)
      .ok_or_else(|| SubqueryError::Custom(format!("Wrong repository url: {}", url)))?;
    let repo = parts1
      .get(len - 1)
      .ok_or_else(|| SubqueryError::Custom(format!("Wrong repository url: {}", url)))?;
    Ok(format!("{}/{}", org, repo))
  }

  async fn project_repo_name(&self, key: impl AsRef<str>) -> color_eyre::Result<String> {
    let project = self
      .project(key.as_ref())
      .await?
      .ok_or_else(|| SubqueryError::Custom(format!("The project {} not found", key.as_ref())))?;
    let repository = project.git_repository.ok_or_else(|| {
      SubqueryError::Custom(format!(
        "Not have git repository url for project {}",
        key.as_ref()
      ))
    })?;
    let repo_name = self.parse_github_repo(repository)?;
    Ok(repo_name)
  }

  fn deserialize<T: DeserializeOwned>(
    &self,
    api: impl AsRef<str>,
    json: impl AsRef<str>,
  ) -> color_eyre::Result<T> {
    let json = json.as_ref();
    if json.starts_with('{') {
      let value: serde_json::Value = serde_json::from_str(json)?;
      if let Some(sc) = value.get("statusCode") {
        return Err(
          SubqueryError::Api(
            api.as_ref().to_string(),
            sc.as_u64().unwrap_or(u64::MAX),
            value
              .get("message")
              .map(|v| v.as_str().unwrap_or("Unknown error").to_string())
              .unwrap_or_else(|| "No message from server".to_string()),
          )
          .into(),
        );
      }
      if let Some(msg) = value.get("message") {
        return Err(
          SubqueryError::Api(
            api.as_ref().to_string(),
            0,
            msg
              .as_str()
              .map(|v| v.to_string())
              .unwrap_or_else(|| "No message from server".to_string()),
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
    let api = "/user";
    let response = self
      .request(Method::GET, api)?
      .header("Cookie", format!("connect.sid={}", sid.as_ref()))
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(api, response)
  }

  pub async fn create_project(
    &self,
    project: Project,
  ) -> color_eyre::Result<CreateProjectResponse> {
    let api = "/subqueries";
    let response = self
      .request(Method::POST, api)?
      .json(&project)
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(api, response)
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
    let api = format!("/user/projects?account={}", account);
    let response = self
      .request(Method::GET, &api)?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(api, response)
  }

  pub async fn project(&self, key: impl AsRef<str>) -> color_eyre::Result<Option<Project>> {
    // https://api.subquery.network/subqueries/fewensa/pangolin-test
    let api = format!("/subqueries/{}", key.as_ref());
    let response = self
      .request(Method::GET, &api)?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(api, response)
  }

  pub async fn deployments(&self, key: impl AsRef<str>) -> color_eyre::Result<Vec<Deployment>> {
    let api = format!("/subqueries/{}/deployments", key.as_ref());
    let response = self
      .request(Method::GET, &api)?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(api, response)
  }

  pub async fn branches(&self, key: impl AsRef<str>) -> color_eyre::Result<Vec<Branch>> {
    let repo_name = self.project_repo_name(key).await?;
    let api = format!("/info/github/{}/branches", repo_name);
    let response = self
      .request(Method::GET, &api)?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(api, response)
  }

  pub async fn image(&self) -> color_eyre::Result<Image> {
    let api = "/info/images";
    let response = self
      .request(Method::GET, &api)?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(api, response)
  }

  pub async fn commit(
    &self,
    key: impl AsRef<str>,
    branch: impl AsRef<str>,
  ) -> color_eyre::Result<Vec<Commit>> {
    let repo_name = self.project_repo_name(key).await?;
    let api = format!("/info/github/{}/commits/{}", repo_name, branch.as_ref());
    let response = self
      .request(Method::GET, &api)?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(api, response)
  }

  pub async fn deploy(
    &self,
    key: impl AsRef<str>,
    data: &CreateDeployRequest,
  ) -> color_eyre::Result<Project> {
    let api = format!("/subqueries/{}/deployments", key.as_ref());
    /*
    {
      "version":"522ac5c1e9948c5b9395a9d429e43565685e7bf7",
      "dictEndpoint":"",
      "indexerImageVersion":"v0.25.3",
      "queryImageVersion":"v0.8.0",
      "type":"stage"
    }
    {
      "version": "522ac5c1e9948c5b9395a9d429e43565685e7bf7",
      "endpoint": "",
      "dictEndpoint": "",
      "indexerImageVersion": "v0.25.3",
      "queryImageVersion": "v0.8.0",
      "type": "stage",
      "subFolder": ""
    }
     */
    let response = self
      .request(Method::POST, &api)?
      .json(data)
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(api, response)
  }

  pub async fn redeploy(
    &self,
    key: impl AsRef<str>,
    id: u64,
    data: &CreateDeployRequest,
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
    let api = format!(
      "/subqueries/{}/deployments/{}/sync-status",
      key.as_ref(),
      id
    );
    let response = self
      .request(Method::GET, &api)?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(api, response)
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
    let api = format!(
      "/subqueries/{}/logs?stage={}&level={}{}",
      key.as_ref(),
      stage,
      level.as_ref(),
      if keyword.is_some() {
        format!("&keyword={}", keyword.unwrap())
      } else {
        Default::default()
      }
    );
    let response = self
      .request(Method::GET, &api)?
      .send()
      .await?
      .text()
      .await?;
    self.deserialize(api, response)
  }
}
