use crate::exporter_type::{ExporterType, determine_exporter_type};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_gcloud_trace::GcpCloudTraceExporterBuilder;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, trace::RandomIdGenerator};
use std::time::Duration;

pub async fn init() -> Result<
    (
        opentelemetry_sdk::trace::SdkTracerProvider,
        opentelemetry_sdk::trace::Tracer,
    ),
    Box<dyn std::error::Error>,
> {
    let exporter_type = determine_exporter_type();

    match exporter_type {
        ExporterType::CloudTrace => {
            let gcp_project_id = std::env::var("GOOGLE_CLOUD_PROJECT")
                .unwrap_or_else(|_| "your-gcp-project-id".to_string());
            tracing::info!("Exporter: Cloud Trace");
            tracing::info!("Google Cloud Project ID: {}", gcp_project_id);
            let gcp_trace_exporter =
                GcpCloudTraceExporterBuilder::new(std::env::var("GOOGLE_CLOUD_PROJECT")?);

            let gcp_tracer_provider = gcp_trace_exporter.create_provider().await?;
            let tracer = gcp_trace_exporter.install(&gcp_tracer_provider).await?;
            Ok((gcp_tracer_provider, tracer))
        }
        _ => {
            let jaeger_endpoint = std::env::var("JAEGER_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:4317".to_string());
            tracing::info!("Exporter: Jaeger");
            tracing::info!("Endpoint: {}", jaeger_endpoint);

            let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint(jaeger_endpoint)
                .with_timeout(Duration::from_secs(3))
                .build()
                .unwrap();

            let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
                .with_batch_exporter(otlp_exporter)
                .with_resource(
                    Resource::builder()
                        .with_service_name("rust-otel-gcloud-example")
                        .build(),
                )
                .with_id_generator(RandomIdGenerator::default())
                .build();
            let tracer = provider.tracer("rust-otel-gcloud-example");
            Ok((provider, tracer))
        }
    }
}
