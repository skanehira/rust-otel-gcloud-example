mod exporter_type;
mod tracer;

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let (provider, tracer) = tracer::init().await?;

    let fmt_layer = fmt::layer()
        .event_format(fmt::format::json())
        .fmt_fields(fmt::format::JsonFields::new());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    tracing::info!("Logging and tracing initialized");

    tracing::info!("Starting the application");

    do_work().await;

    tracing::info!("Shutting down the application");

    provider.shutdown()?;

    Ok(())
}

// Example of a traced function
#[tracing::instrument(level = "info", fields(function_type = "worker"))]
async fn do_work() {
    tracing::info!("Starting work");
    tracing::debug!("Work debug level log");
    tracing::error!("Work error level log");
    tracing::warn!("Work warning level log");
    // Simulate someworksing
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    nested_work().await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    tracing::info!("Work completed");
}

#[tracing::instrument(level = "info", fields(function_type = "nested"))]
async fn nested_work() {
    tracing::info!("Starting nested work");
    // Simulate some processing
    tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
    tracing::debug!("Nested work debug level log");
    tracing::error!("Nested work error level log");
    tracing::warn!("Nested work warning level log");
    tracing::info!("Nested work completed");
}
