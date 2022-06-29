use opentelemetry::{
    sdk::{
        trace::{Config, Tracer},
        Resource,
    },
    trace::TraceError,
    KeyValue,
};

/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
pub fn init_tracing() -> Result<Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline()
        .with_service_name("mari")
        .with_trace_config(Config::default().with_resource(Resource::new(vec![
            KeyValue::new("service.name", "mari-service"),
            KeyValue::new("exporter", "otlp-jaeger"),
        ])))
        .install_batch(opentelemetry::runtime::Tokio)
}
