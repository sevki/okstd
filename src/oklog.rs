use std::error::Error;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt;

pub fn setup_logging(level: LevelFilter) -> Result<(), Box<dyn Error>> {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = fmt::Subscriber::builder()
        .with_max_level(level)
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
