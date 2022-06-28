use nalgebra::base::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Twist {
    pub linear: Vector3<f64>,
    pub angular: Vector3<f64>,
}
