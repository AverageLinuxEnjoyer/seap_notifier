use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractObject {
    pub subscription_id: u32,
    pub value: String,
}
