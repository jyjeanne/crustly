use anyhow::Result;
use crustly::cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to file to avoid interfering with TUI
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("crustly")
        .join("logs");

    // Create log directory if it doesn't exist
    std::fs::create_dir_all(&log_dir).ok();

    let file_appender = tracing_appender::rolling::daily(log_dir, "crustly.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    // Parse CLI arguments and run
    cli::run().await
}
