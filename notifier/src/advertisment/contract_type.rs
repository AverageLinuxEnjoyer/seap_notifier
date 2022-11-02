use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ContractType {
    Furnizare,
    Servicii,
    Lucrari,
}
