# Rust OpenTelemetry + Jaeger/Cloud Trace Sample

This project is a sample code that collects trace logs using OpenTelemetry in Rust and sends them to Jaeger or Google Cloud Trace. It sends trace data using the OTLP gRPC protocol.

## Features

- Collection of traces using the OpenTelemetry SDK
- Sending trace data to Jaeger using the OTLP gRPC protocol
- Sending trace data to Google Cloud Trace
- Standard output logs in JSON format
- Integration of tracing and logging
- Exporter selection feature via environment variables

## Prerequisites

- Rust (latest version recommended)
- Docker and Docker Compose (when running with Docker Compose)
- GCP project and authentication settings when using Google Cloud Trace

## Execution Methods

### Method 1: Send to Jaeger in a Local Environment

1. Start Jaeger locally:
   ```bash
   docker compose up -d
   ```

2. Build and run the application:
   ```bash
   OTEL_EXPORTER=jaeger JAEGER_ENDPOINT=http://localhost:4317 RUST_LOG=info cargo run
   ```

3. Access the Jaeger UI: http://localhost:16686

### Method 2: Send to Google Cloud Trace

1. GCP authentication settings:
   
   ```bash
   # Set up ADC using gcloud
   gcloud auth application-default login
   ```

2. Run the application:
   ```bash
   OTEL_EXPORTER=cloud_trace GOOGLE_CLOUD_PROJECT=your-project-id cargo run
   ```

## Environment Variables

- `RUST_LOG`: Control the log level (e.g., `info`, `debug`, `warn`)
- `OTEL_EXPORTER`: Specify the exporter to use (`jaeger` or `cloud_trace`, default: `jaeger`)
- `JAEGER_ENDPOINT`: Jaeger's OTLP endpoint (default: `http://localhost:4317`)
- `GOOGLE_CLOUD_PROJECT`: Google Cloud Project ID
