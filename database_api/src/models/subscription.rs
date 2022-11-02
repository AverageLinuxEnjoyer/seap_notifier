use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscription {
    pub id: u32,
    pub id_user: u32,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
}
