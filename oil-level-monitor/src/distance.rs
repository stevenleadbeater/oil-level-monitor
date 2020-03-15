use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Distance {
    pub id: i32,
    pub distance: i32,
}