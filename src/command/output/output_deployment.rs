use colored::Colorize;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

use crate::command::types::OutputFormat;
use crate::subquery::{Deployment, DeploymentStatus, DeploymentType};

pub fn output_deployment(
  deployments: Vec<Deployment>,
  format: OutputFormat,
) -> color_eyre::Result<()> {
  match format {
    OutputFormat::Raw => output_raw(deployments),
    OutputFormat::Json => output_json(deployments),
    OutputFormat::Table => output_table(deployments),
  }
}

fn output_json(deployments: Vec<Deployment>) -> color_eyre::Result<()> {
  println!("{}", serde_json::to_string_pretty(&deployments)?);
  Ok(())
}

fn output_raw(deployments: Vec<Deployment>) -> color_eyre::Result<()> {
  let primary = deployments
    .iter()
    .find(|item| item.type_ == DeploymentType::Primary);
  let stage = deployments
    .iter()
    .find(|item| item.type_ == DeploymentType::Stage);
  if primary.is_none() && stage.is_none() {
    println!("Not have any deployments");
  }
  if let Some(primary) = primary {
    println!("{}", "Primary".bold().blue());
    _output_raw_deployment(primary)?;
    println!();
  }
  if let Some(stage) = stage {
    println!("{}", "Stage".bold().blue());
    _output_raw_deployment(stage)?;
  }
  Ok(())
}

fn output_table(deployments: Vec<Deployment>) -> color_eyre::Result<()> {
  output_raw(deployments)
}

fn _output_raw_deployment(deployment: &Deployment) -> color_eyre::Result<()> {
  let mut table = Table::new();
  table.max_column_width = 40;
  table.separate_rows = false;
  table.style = TableStyle::empty();
  table.add_row(Row::new(vec![
    TableCell::new("Name".bold()),
    TableCell::new_with_alignment("Value".bold(), 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Id".bold()),
    TableCell::new_with_alignment(deployment.id, 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Project key".bold()),
    TableCell::new_with_alignment(&deployment.project_key, 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Commit".bold()),
    TableCell::new_with_alignment(&deployment.version, 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Status".bold()),
    TableCell::new_with_alignment(
      match deployment.status {
        DeploymentStatus::Error => "Error".bold().red(),
        DeploymentStatus::Running => "Running".bold().green(),
        DeploymentStatus::Processing => "Processing".bold().cyan(),
      },
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Cluster".bold()),
    TableCell::new_with_alignment(&deployment.cluster, 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Indexer image".bold()),
    TableCell::new_with_alignment(&deployment.indexer_image, 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Query image".bold()),
    TableCell::new_with_alignment(&deployment.query_image, 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Sub folder".bold()),
    TableCell::new_with_alignment(
      deployment.sub_folder.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Endpoint".bold()),
    TableCell::new_with_alignment(
      deployment.endpoint.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Dict endpoint".bold()),
    TableCell::new_with_alignment(
      deployment
        .dict_endpoint
        .clone()
        .unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Query url".bold()),
    TableCell::new_with_alignment(&deployment.query_url, 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Query cluster url".bold()),
    TableCell::new_with_alignment(&deployment.query_cluster_url, 2, Alignment::Left),
  ]));

  println!("{}", table.render());
  Ok(())
}
