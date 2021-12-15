use reqwest::{Client, Method, RequestBuilder};

#[derive(Clone, Debug)]
pub struct Subquery {
  client: Client,
  endpoint: String,
  access_token: Option<String>,
}

impl Subquery {
  pub fn new(endpoint: impl AsRef<str>) -> color_eyre::Result<Self> {
    let client = Client::builder()
      .timeout(std::time::Duration::from_secs(10))
      .build()?;
    Ok(Self {
      client,
      endpoint: endpoint.as_ref().to_string(),
      access_token: None,
    })
  }
}

impl Subquery {
  fn api(&self, api: impl AsRef<str>) -> String {
    format!("{}{}", self.endpoint, api.as_ref())
  }

  fn request(&self, method: Method, api: impl AsRef<str>) -> color_eyre::Result<RequestBuilder> {
    let api = self.api(api);
    let mut builder = self.client.request(method, api);
    builder = match &self.access_token {
      Some(token) => builder.bearer_auth(token),
      None => builder,
    };
    Ok(builder)
  }
}

impl Subquery {
  pub async fn access_token(&self) -> color_eyre::Result<()> {
    // let response = self.client.get("/user").send().await?.text().await?;
    let response = self
      .request(Method::GET, "/ser")?
      .send()
      .await?
      .text()
      .await?;
    tracing::info!("===> {}", response);
    Ok(())
  }
}
