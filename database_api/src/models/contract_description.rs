use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractDescription {
    pub subscription_id: u32,
    pub value: String,
}
