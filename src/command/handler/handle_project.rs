use crate::command::types::{OutputFormat, ProjectOpt};
use crate::subquery::{ApiVersion, Project};
use crate::Subquery;

pub async fn handle_project(subquery: &Subquery, opt: ProjectOpt) -> color_eyre::Result<()> {
  match opt {
    ProjectOpt::Create {
      org,
      key,
      name,
      subtitle,
      description,
      repo,
      hide,
      check,
    } => {
      let project = Project {
        api_version: ApiVersion::Latest,
        created_at: None,
        updated_at: None,
        key: format!("{}/{}", org, key),
        account: Some(org),
        name: Some(name.unwrap_or(key)),
        network: None,
        deployed: None,
        logo_url: None,
        subtitle,
        description,
        git_repository: Some(repo),
        hide: Some(hide.unwrap_or(true)),
        dedicate_db_key: None,
        query_url: None,
        deployment: None,
      };
      handle_create(subquery, project, check).await
    }
    ProjectOpt::Update {
      org,
      key,
      name,
      subtitle,
      description,
      hide,
    } => {
      let project = Project {
        api_version: ApiVersion::Latest,
        created_at: None,
        updated_at: None,
        key: format!("{}/{}", org, key),
        account: None,
        name,
        network: None,
        deployed: None,
        logo_url: None,
        subtitle,
        description,
        git_repository: None,
        hide,
        dedicate_db_key: None,
        query_url: None,
        deployment: None,
      };
      handle_update(subquery, project).await
    }
    ProjectOpt::Delete { org, key } => handle_delete(subquery, format!("{}/{}", org, key)).await,
    ProjectOpt::List { org, output } => handle_list(subquery, org, output).await,
  }
}

async fn handle_delete(subquery: &Subquery, key: impl AsRef<str>) -> color_eyre::Result<()> {
  let question = requestty::Question::expand("delete")
    .message("Are you sure delete this project?")
    .choices(vec![('y', "Yes"), ('n', "No")])
    .default_separator()
    .choice('x', "Abort")
    .build();
  let answer = requestty::prompt_one(question)?;
  if let Some(v) = answer.as_expand_item() {
    if v.key != 'y' {
      return Ok(());
    }
    let _response = subquery.delete_project(key.as_ref()).await?;
    println!("Success");
  }
  Ok(())
}

async fn handle_create(
  subquery: &Subquery,
  project: Project,
  check: bool,
) -> color_eyre::Result<()> {
  if check {
    let projects = subquery
      .projects(project.account.clone().expect("unreachable"))
      .await?;
    let this_key_project = projects.iter().find(|&item| item.key == project.key);
    if this_key_project.is_some() {
      println!("{}", project.key);
      return Ok(());
    }
  }
  let response = subquery.create_project(project).await?;
  println!("{}", response.key);
  Ok(())
}

async fn handle_update(subquery: &Subquery, project: Project) -> color_eyre::Result<()> {
  let _response = subquery.update_project(project).await?;
  println!("Success");
  Ok(())
}

async fn handle_list(
  subquery: &Subquery,
  org: String,
  format: OutputFormat,
) -> color_eyre::Result<()> {
  let projects = subquery.projects(org).await?;
  match format {
    OutputFormat::Raw | OutputFormat::Table => {
      projects.iter().for_each(|project| {
        let key_name = project.key.split('/').last();
        let project_name = project.name.clone().unwrap_or_default();
        if key_name.unwrap_or("") == project_name {
          println!("{}", project.key)
        } else {
          println!("{} ({})", project.key, project_name)
        }
      });
    }
    OutputFormat::Json => {
      let json = serde_json::to_string_pretty(&projects)?;
      println!("{}", json);
    }
  }
  Ok(())
}
