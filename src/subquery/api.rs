#![allow(dead_code)]

use reqwest::{Client, Method, RequestBuilder};

use crate::subquery::{
  Branch, Commit, CreateProjectResponse, DeployRequest, Deployment, Image, Project, SyncStatus,
  User,
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
    Ok(serde_json::from_str(&response)?)
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
    Ok(serde_json::from_str(&response)?)
  }

  pub async fn update_project(&self, project: Project) -> color_eyre::Result<()> {
    let _response = self
      .request(Method::PUT, format!("/subqueries/{}", project.key))?
      .json(&project)
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
    Ok(serde_json::from_str(&response)?)
  }

  pub async fn project(&self, key: impl AsRef<str>) -> color_eyre::Result<Option<Project>> {
    // https://api.subquery.network/subqueries/fewensa/pangolin-test
    let response = self
      .request(Method::GET, format!("/subqueries/{}", key.as_ref()))?
      .send()
      .await?
      .text()
      .await?;
    Ok(serde_json::from_str(&response)?)
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
    Ok(serde_json::from_str(&response)?)
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
    Ok(serde_json::from_str(&response)?)
  }

  pub async fn image(&self) -> color_eyre::Result<Image> {
    let response = self
      .request(Method::GET, "/info/images")?
      .send()
      .await?
      .text()
      .await?;
    Ok(serde_json::from_str(&response)?)
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
    Ok(serde_json::from_str(&response)?)
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
    Ok(serde_json::from_str(&response)?)
  }

  pub async fn redeploy(
    &self,
    key: impl AsRef<str>,
    id: u64,
    data: &DeployRequest,
  ) -> color_eyre::Result<()> {
    let response = self
      .request(
        Method::PUT,
        format!("/subqueries/{}/deployments/{}", key.as_ref(), id),
      )?
      .json(data)
      .send()
      .await?
      .text()
      .await?;
    serde_json::from_str(&response)?;
    Ok(())
  }

  pub async fn delete_deploy(&self, key: impl AsRef<str>, id: u64) -> color_eyre::Result<()> {
    let response = self
      .request(
        Method::DELETE,
        format!("/subqueries/{}/deployments/{}", key.as_ref(), id),
      )?
      .send()
      .await?
      .text()
      .await?;
    serde_json::from_str(&response)?;
    Ok(())
  }

  pub async fn rebase_deployment(&self, key: impl AsRef<str>, id: u64) -> color_eyre::Result<()> {
    let response = self
      .request(
        Method::POST,
        format!("/subqueries/{}/deployments/{}/release", key.as_ref(), id),
      )?
      .send()
      .await?
      .text()
      .await?;
    serde_json::from_str(&response)?;
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
    Ok(serde_json::from_str(&response)?)
  }
}
