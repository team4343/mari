use color_eyre::eyre::Result;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};

/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
pub fn init_tracing() -> Result<()> {
    std::env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "12");

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("mari")
        .with_max_packet_size(2usize.pow(13))
        .install_batch(opentelemetry::runtime::Tokio)?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()?;

    Ok(())
}
