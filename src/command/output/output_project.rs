use colored::Colorize;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

use crate::command::types::OutputFormat;
use crate::subquery::Project;

pub fn output_project(project: Project, format: OutputFormat) -> color_eyre::Result<()> {
  match format {
    OutputFormat::Raw => output_raw(project),
    OutputFormat::Json => output_json(project),
    OutputFormat::Table => output_table(project),
  }
}

fn output_raw(project: Project) -> color_eyre::Result<()> {
  println!("{}", &project.key.bold().blue());
  let mut table = Table::new();
  table.max_column_width = 40;
  table.separate_rows = false;
  table.style = TableStyle::empty();
  table.add_row(Row::new(vec![
    TableCell::new("Name".bold()),
    TableCell::new_with_alignment("Value".bold(), 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Key".bold()),
    TableCell::new_with_alignment(project.key, 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Account".bold()),
    TableCell::new_with_alignment(
      project.account.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Name".bold()),
    TableCell::new_with_alignment(
      project.name.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Network".bold()),
    TableCell::new_with_alignment(
      project.network.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Deployed".bold()),
    TableCell::new_with_alignment(
      project.deployed.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Logo".bold()),
    TableCell::new_with_alignment(
      project.logo_url.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Subtitle".bold()),
    TableCell::new_with_alignment(
      project.subtitle.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Description".bold()),
    TableCell::new_with_alignment(
      project.subtitle.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Repository".bold()),
    TableCell::new_with_alignment(
      project.git_repository.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Hide".bold()),
    TableCell::new_with_alignment(
      project.hide.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Dedicate db key".bold()),
    TableCell::new_with_alignment(
      project
        .dedicate_db_key
        .clone()
        .unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("Query url".bold()),
    TableCell::new_with_alignment(
      project.query_url.clone().unwrap_or(Default::default()),
      2,
      Alignment::Left,
    ),
  ]));
  println!("{}", table.render());
  println!();
  if let Some(deployment) = project.deployment {
    let deployments = vec![deployment];
    crate::command::output::output_deployment(deployments, OutputFormat::Raw)?;
  }
  Ok(())
}

fn output_json(project: Project) -> color_eyre::Result<()> {
  println!("{}", serde_json::to_string_pretty(&project)?);
  Ok(())
}

fn output_table(project: Project) -> color_eyre::Result<()> {
  output_raw(project)
}
