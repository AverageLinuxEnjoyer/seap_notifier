use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignedUser {
    id: u32,
    text: String,

    #[serde(rename = "localeKey")]
    locale_key: Option<String>,
}
