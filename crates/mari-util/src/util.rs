use color_eyre::eyre::InstallError;
use opentelemetry::{
    global,
    sdk::{
        propagation::TraceContextPropagator,
        trace::{Config, Tracer},
        Resource,
    },
    trace::TraceError,
    KeyValue,
};
use tracing_subscriber::{prelude::*, util::TryInitError};
use tracing_tree::HierarchicalLayer;

pub fn setup_opentelemetry_jaeger(service_name: String) -> Result<Tracer, TraceError> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    opentelemetry_jaeger::new_pipeline()
        .with_service_name("mari-dashboard")
        .with_trace_config(Config::default().with_resource(Resource::new(vec![
            KeyValue::new("service.name", service_name),
            KeyValue::new("exporter", "otlp-jaeger"),
        ])))
        .install_batch(opentelemetry::runtime::Tokio)
}

pub fn setup_tracing_subscriber(tracer: Tracer) -> Result<Tracer, TryInitError> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(
            HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .with(tracing_error::ErrorLayer::default())
        .with(tracing_opentelemetry::layer().with_tracer(tracer.clone()))
        .try_init()
        .map(|_| tracer)
}

pub fn setup_eyre_traced_hooks() -> Result<(), InstallError> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();
    eyre_hook.install()?;
    std::panic::set_hook(Box::new(move |pi| {
        tracing::error_span!("error")
            .in_scope(|| tracing::error!("{}", panic_hook.panic_report(pi)));
        teardown();
    }));

    Ok(())
}

pub fn teardown() {
    global::shutdown_tracer_provider()
}
