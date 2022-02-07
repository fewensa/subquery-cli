use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::{Table, TableStyle};

use crate::command::types::UserOpt;
use crate::Subquery;

pub async fn handle_user(subquery: &Subquery, opt: UserOpt) -> color_eyre::Result<()> {
  match opt {
    UserOpt::Info => handle_user_info(subquery).await,
    UserOpt::Orgs => handle_orgs(subquery).await,
  }
}

async fn handle_user_info(subquery: &Subquery) -> color_eyre::Result<()> {
  let user = subquery.user().await?;
  let mut table = Table::new();
  table.max_column_width = 40;
  table.separate_rows = false;
  table.style = TableStyle::blank();
  table.add_row(Row::new(vec![
    TableCell::new("ID"),
    TableCell::new_with_alignment(user.id, 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("NAME"),
    TableCell::new_with_alignment(user.username, 2, Alignment::Left),
  ]));
  table.add_row(Row::new(vec![
    TableCell::new("EMAIL"),
    TableCell::new_with_alignment(user.email, 2, Alignment::Left),
  ]));
  println!("{}", table.render());
  Ok(())
}

async fn handle_orgs(subquery: &Subquery) -> color_eyre::Result<()> {
  let user = subquery.user().await?;
  for account in &user.accounts {
    println!("{}", account.key)
  }
  Ok(())
}
