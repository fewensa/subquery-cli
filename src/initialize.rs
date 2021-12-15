use tracing::Level;

pub fn init() -> color_eyre::Result<()> {
  init_log()?;
  Ok(())
}

fn init_log() -> color_eyre::Result<()> {
  color_eyre::install()?;
  if std::env::var("RUST_SPANTRACE").is_err() {
    std::env::set_var("RUST_SPANTRACE", "1");
  }

  let subscriber = tracing_subscriber::FmtSubscriber::builder()
    // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
    // will be written to stdout.
    .with_max_level(Level::TRACE)
    .with_env_filter("trace,hyper=error")
    // builds the subscriber.
    .finish();

  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
  Ok(())
}
