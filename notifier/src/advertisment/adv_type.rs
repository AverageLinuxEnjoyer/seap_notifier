use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AdvType {
    #[serde(rename = "Achizitii directe")]
    AchitiziiDirecte,

    #[serde(rename = "Anexa 2")]
    Anexa2,
}
