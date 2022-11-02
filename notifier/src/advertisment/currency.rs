use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub id: u32,
    pub text: String,

    #[serde(rename = "localeKey")]
    pub locale_key: Option<String>,
}
