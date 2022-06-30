use mari_protocol::{health_server::Health, Ping};
use tonic::{Request, Response, Status};
use wpilib::robot::{error::RobotError, UninitializedRobot};

pub struct Robot {
    wpilib: wpilib::robot::Robot,
}

impl Robot {
    pub fn try_new() -> Result<Self, RobotError> {
        let wpilib = UninitializedRobot::default().initialize()?;
        Ok(Self { wpilib })
    }
}

#[tonic::async_trait]
impl Health for Robot {
    async fn ping(&self, _request: Request<Ping>) -> Result<Response<Ping>, Status> {
        Ok(Response::new(Ping {}))
    }
}
