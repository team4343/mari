pub(crate) mod util;

use mari_protocol::{
    health_server::{Health, HealthServer},
    Ping,
};
use tonic::{transport::Server, Request, Response, Status};
use util::init_tracing;

#[derive(Default)]
pub struct Robot;

#[tonic::async_trait]
impl Health for Robot {
    async fn ping(&self, _request: Request<Ping>) -> Result<Response<Ping>, Status> {
        Ok(Response::new(Ping {}))
    }
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    let _tracer = init_tracing()?;

    let robot = Robot::default();

    let addr = "[::1]:50051".parse().unwrap();

    Server::builder()
        .add_service(HealthServer::new(robot))
        .serve(addr)
        .await?;

    Ok(())
}
