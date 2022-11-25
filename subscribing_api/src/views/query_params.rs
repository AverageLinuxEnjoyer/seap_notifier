use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Email {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub start_index: u32,
    pub count: u32,
}
