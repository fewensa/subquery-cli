use crate::command::types::{DeploymentOpt, OutputFormat};
use crate::subquery::{CreateDeployRequest, DeploymentType};
use crate::{Subquery, SubqueryError};

pub async fn handle_deployment(subquery: &Subquery, opt: DeploymentOpt) -> color_eyre::Result<()> {
  match opt {
    DeploymentOpt::List { org, key, output } => {
      handle_list(subquery, format!("{}/{}", org, key), output).await
    }
    DeploymentOpt::Deploy {
      org,
      key,
      output,
      command,
      force,
    } => {
      let deployment = CreateDeployRequest {
        commit: command.commit,
        endpoint: command.endpoint,
        dict_endpoint: command.dict_endpoint,
        indexer_image_version: command.indexer_image_version,
        query_image_version: command.query_image_version,
        type_: command.type_,
        sub_folder: command.sub_folder,
      };
      handle_deploy(
        subquery,
        format!("{}/{}", org, key),
        command.branch,
        deployment,
        output,
        force,
      )
      .await
    }
    DeploymentOpt::Delete { org, key, id } => {
      handle_delete(subquery, format!("{}/{}", org, key), id).await
    }
    DeploymentOpt::Redeploy {
      org,
      key,
      id,
      command,
    } => {
      let deployment = CreateDeployRequest {
        commit: command.commit,
        endpoint: command.endpoint,
        dict_endpoint: command.dict_endpoint,
        indexer_image_version: command.indexer_image_version,
        query_image_version: command.query_image_version,
        type_: command.type_,
        sub_folder: command.sub_folder,
      };
      handle_redeploy(
        subquery,
        format!("{}/{}", org, key),
        command.branch,
        id,
        deployment,
      )
      .await
    }
    DeploymentOpt::Promote { org, key, id } => {
      handle_promote(subquery, format!("{}/{}", org, key), id).await
    }
    DeploymentOpt::SyncStatus {
      org,
      key,
      id,
      rolling,
      interval,
    } => handle_sync_status(subquery, format!("{}/{}", org, key), id, rolling, interval).await,
  }
}

async fn handle_sync_status(
  subquery: &Subquery,
  key: impl AsRef<str>,
  id: u64,
  rolling: bool,
  interval: u64,
) -> color_eyre::Result<()> {
  let mut times = 0usize;
  loop {
    times += 1;
    let status = subquery.deployment_sync_status(key.as_ref(), id).await?;
    let percent = (status.processing_block as f32 / status.target_block as f32) * 100f32;
    println!(
      "total_entities: {} target_block: {} processing_block: {} percent: {}%{} ",
      status.total_entities,
      status.target_block,
      status.processing_block,
      format!("{:.2}", percent),
      if rolling {
        format!(" [{}]", times)
      } else {
        "".to_string()
      },
    );
    if !rolling {
      break;
    }
    tokio::time::sleep(std::time::Duration::from_secs(interval)).await
  }
  Ok(())
}

async fn handle_promote(
  subquery: &Subquery,
  key: impl AsRef<str>,
  id: Option<u64>,
) -> color_eyre::Result<()> {
  let key = key.as_ref();
  if id.is_some() {
    let _ = subquery.rebase_deployment(key, id.unwrap()).await?;
    println!("Success");
    return Ok(());
  }
  let deployments = subquery.deployments(key).await?;
  let latest_stage_deployment = deployments
    .iter()
    .find(|&item| item.type_ == DeploymentType::Stage);
  if let Some(deployment) = latest_stage_deployment {
    let _ = subquery.rebase_deployment(key, deployment.id).await?;
    println!("Success");
    return Ok(());
  }
  eprintln!("Not found any stage deployment");
  Ok(())
}

async fn handle_redeploy(
  subquery: &Subquery,
  key: impl AsRef<str>,
  branch: impl AsRef<str>,
  id: Option<u64>,
  mut deployment: CreateDeployRequest,
) -> color_eyre::Result<()> {
  let key = key.as_ref();
  if id.is_some() {
    deployment = safe_create_deploy(subquery, deployment, key, branch).await?;
    let _response = subquery.redeploy(key, id.unwrap(), &deployment).await?;
    println!("Success");
    return Ok(());
  }

  let type_ = &deployment.type_;
  let deployments = subquery.deployments(key).await?;
  let this_type_latest_deployment = deployments.iter().find(|&item| &item.type_ == type_);
  if let Some(latest) = this_type_latest_deployment {
    deployment = safe_create_deploy(subquery, deployment, key, branch).await?;
    let _response = subquery.redeploy(key, latest.id, &deployment).await?;
    println!("Success");
    return Ok(());
  }
  eprintln!("Not found any deploy for type: {:?}", type_);
  std::process::exit(1)
}

async fn handle_delete(
  subquery: &Subquery,
  key: impl AsRef<str>,
  id: u64,
) -> color_eyre::Result<()> {
  let question = requestty::Question::expand("delete")
    .message("Are you sure delete this deployment?")
    .choices(vec![('y', "Yes"), ('n', "No")])
    .default_separator()
    .choice('x', "Abort")
    .build();
  let answer = requestty::prompt_one(question)?;
  if let Some(v) = answer.as_expand_item() {
    if v.key != 'y' {
      return Ok(());
    }
    let _response = subquery.delete_deploy(key, id).await?;
    println!("Success");
  }
  Ok(())
}

async fn handle_list(
  subquery: &Subquery,
  key: impl AsRef<str>,
  output: OutputFormat,
) -> color_eyre::Result<()> {
  let deployments = subquery.deployments(key).await?;
  crate::command::output::output_deployment(deployments, output)?;
  Ok(())
}

async fn handle_deploy(
  subquery: &Subquery,
  key: impl AsRef<str>,
  branch: impl AsRef<str>,
  mut deployment: CreateDeployRequest,
  output: OutputFormat,
  force: bool,
) -> color_eyre::Result<()> {
  if force {
    let deployments = subquery.deployments(key.as_ref()).await?;
    let this_type_deployment = deployments
      .iter()
      .find(|&item| item.type_ == deployment.type_);
    if let Some(deployment) = this_type_deployment {
      tracing::info!("In force mode, delete deploy for id {}", deployment.id);
      subquery.delete_deploy(key.as_ref(), deployment.id).await?;
    }
  }
  deployment = safe_create_deploy(subquery, deployment, key.as_ref(), branch).await?;
  let response = subquery.deploy(key, &deployment).await?;
  crate::command::output::output_project(response, output)
}

async fn safe_create_deploy(
  subquery: &Subquery,
  mut deployment: CreateDeployRequest,
  key: impl AsRef<str>,
  branch: impl AsRef<str>,
) -> color_eyre::Result<CreateDeployRequest> {
  // commit
  if deployment.commit.is_none() {
    let commits = subquery.commit(key.as_ref(), branch.as_ref()).await?;
    match commits.first() {
      Some(c) => {
        deployment.commit = Some(c.sha.clone());
      }
      None => {
        let project = subquery.project(key.as_ref()).await?.ok_or_else(|| {
          SubqueryError::Custom(format!("The project {} not found", key.as_ref()))
        })?;
        let msg = format!(
          "No commit found in git repository {}#{}",
          project.git_repository.unwrap_or_default(),
          branch.as_ref()
        );
        return Err(SubqueryError::Custom(msg).into());
      }
    }
  }

  // image version
  let not_set_image =
    deployment.query_image_version.is_none() || deployment.indexer_image_version.is_none();
  let mut image = None;
  if not_set_image {
    image = Some(subquery.image().await?);
  }

  if deployment.query_image_version.is_none() {
    let image = image.clone().unwrap();
    deployment.query_image_version = Some(
      image
        .query
        .get(0)
        .cloned()
        .ok_or_else(|| SubqueryError::Custom("Not found query image".to_string()))?,
    );
  }
  if deployment.indexer_image_version.is_none() {
    let image = image.unwrap();
    deployment.indexer_image_version = Some(
      image
        .indexer
        .get(0)
        .cloned()
        .ok_or_else(|| SubqueryError::Custom("Not found indexer image".to_string()))?,
    );
  }

  Ok(deployment)
}
