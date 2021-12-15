use crate::Subquery;

pub async fn handle_login(subquery: &Subquery, sid: impl AsRef<str>) -> color_eyre::Result<()> {
  let user = subquery.user(sid).await?;
  tracing::trace!(
    "Success to set user to subquery => {}({})",
    user.display_name,
    user.username
  );
  subquery.config().store_user(user)?;
  println!("Success");
  Ok(())
}
