use crate::Subquery;

pub async fn handle_login(subquery: &Subquery, token: impl AsRef<str>) -> color_eyre::Result<()> {
  let user = subquery.user(token).await?;
  tracing::trace!(
    "Success to set user to subquery => {}({})",
    user.display_name,
    user.username
  );
  subquery.config().store_user(user)?;
  println!("Success");
  Ok(())
}
