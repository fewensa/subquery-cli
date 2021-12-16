use crate::command::types::LogsCommand;
use crate::Subquery;

pub async fn handle_logs(subquery: &Subquery, command: LogsCommand) -> color_eyre::Result<()> {
  let key = format!("{}/{}", command.org, command.key);
  let mut viewed = vec![];
  loop {
    let log = subquery
      .search_logs(&key, command.stage, &command.level, command.keyword.clone())
      .await?;
    for ret in log.result {
      let ts = ret.timestamp.timestamp_millis();
      if viewed.contains(&ts) {
        continue;
      }
      viewed.push(ts);
      println!(
        "[{}] [{}] [{}] {} ",
        ret.level, ret.timestamp, ret.category, ret.message
      );
    }
    if !command.rolling {
      break;
    }
    tokio::time::sleep(std::time::Duration::from_secs(command.interval)).await
  }
  Ok(())
}
