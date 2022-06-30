pub mod robot;

use color_eyre::eyre::Result;
use mari_protocol::health_server::HealthServer;
use mari_util::util;
use robot::Robot;
use tonic::transport::Server;
use tracing::info;
#[tracing::instrument]
fn x() {
    info!("This comes from X!");
}

#[tokio::main]
async fn main() -> Result<()> {
    let _tracer = util::setup_tracing_subscriber(util::setup_opentelemetry_jaeger("mari".into())?)?;
    let _hooks = util::setup_eyre_traced_hooks()?;
    info!(action = "set up telemetry", success = true);

    x();

    let robot = Robot::try_new()?;

    let addr = "[::1]:5800".parse().unwrap();
    info!("Serving to {addr}...");
    Server::builder()
        .add_service(HealthServer::new(robot))
        .serve(addr)
        .await?;

    util::teardown();

    Ok(())
}
