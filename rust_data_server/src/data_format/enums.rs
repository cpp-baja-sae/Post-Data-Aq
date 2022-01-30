use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub use Axis::{X as Longitude, Y as Latitude, Z as Elevation};
