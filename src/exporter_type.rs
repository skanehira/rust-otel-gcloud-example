pub enum ExporterType {
    Jaeger,
    CloudTrace,
}

impl From<String> for ExporterType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "jaeger" => ExporterType::Jaeger,
            "cloud_trace" => ExporterType::CloudTrace,
            _ => ExporterType::Jaeger,
        }
    }
}

pub fn determine_exporter_type() -> ExporterType {
    std::env::var("OTEL_EXPORTER")
        .map(Into::into)
        .unwrap_or(ExporterType::Jaeger)
}
