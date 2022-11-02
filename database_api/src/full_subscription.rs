use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FullSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub email: String,

    pub min_value: Option<f32>,

    pub max_value: Option<f32>,

    // nume
    pub contract_object: Option<Vec<String>>,

    // descriere
    pub contract_desc: Option<Vec<String>>,
    // conditii referitoare la contract
    // pub contract_related_conditions: Option<String>,

    // conditii participare
    // pub participation_conditions: Option<String>,

    // informatii suplimentare
    // pub additional_information: Option<String>,

    // cod si denumire cpv
    // pub cpv_code_and_name: Option<String>,
    // tip anunt / sysAdvertisingNotice
    // pub announcement_type:
}
