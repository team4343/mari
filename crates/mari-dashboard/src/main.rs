pub(crate) mod util;

use color_eyre::eyre::Result;
use tonic::Request;
use tracing::info;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};

use crate::util::init_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing()?;

    let mut health =
        mari_protocol::health_client::HealthClient::connect("http://[::1]:50051").await?;

    let request = Request::new(mari_protocol::Ping {});
    let response = health.ping(request).await?;
    info!("{:?}", response);

    Ok(())
}
