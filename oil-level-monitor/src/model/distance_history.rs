use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistanceHistory {
    pub id: Option<i32>,
    pub distance_id: i32,
    pub distance: i32,
    pub time_of_reading: SystemTime,
}
