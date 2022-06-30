use color_eyre::eyre::Result;
use mari_util::util;
use tonic::Request;
use tracing::{info, instrument};

#[instrument]
async fn connect() -> Result<()> {
    info!("Connecting...");

    let mut health =
        mari_protocol::health_client::HealthClient::connect("http://172.22.11.2:5800").await?;

    let request = Request::new(mari_protocol::Ping {});
    let response = health.ping(request).await?;
    info!("{:?}", response);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let _tracer =
        util::setup_tracing_subscriber(util::setup_opentelemetry_jaeger("mari-dashboard".into())?)?;
    let _hooks = util::setup_eyre_traced_hooks()?;

    connect().await?;

    util::teardown();

    Ok(())
}
