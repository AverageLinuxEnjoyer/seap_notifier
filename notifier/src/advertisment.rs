use self::{
    assigned_user::AssignedUser, cpv_code::CpvCode, currency::Currency,
    sys_advertising_notice::SysAdvertisingNotice, sys_notice_state::SysNoticeState,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub mod adv_type;
pub mod assigned_user;
pub mod contract_type;
pub mod cpv_code;
pub mod currency;
pub mod sys_advertising_notice;
pub mod sys_notice_state;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Advertisment {
    pub value: f32,
    pub contract_authority_name: String,
    pub advertising_notice_id: u32,
    pub contracting_authority_id: u32,
    pub assigned_user: AssignedUser,
    pub notice_no: String,
    pub cpv_code: CpvCode,

    pub estimated_value: Option<f32>,
    pub max_estimated_value: Option<f32>,
    pub min_estimated_value: Option<f32>,
    pub currency_id: u32,

    pub notice_entity_address_id: u32,
    pub contract_object: String,
    pub contract_description: String,
    pub contract_related_conditions: Option<String>,
    pub award_criteria: Option<String>,
    pub parent_advertising_notice_id: u32,
    // document ID, URL, Name, UniqueIdentificationCode?
    pub sys_notice_state_id: u32,
    pub sys_notice_state: SysNoticeState,

    pub sys_advertising_notice_id: u32,
    pub sys_advertising_notice: SysAdvertisingNotice,

    #[serde(with = "time::serde::rfc3339")]
    pub publication_date: OffsetDateTime,

    pub currency: Currency,
    pub participation_conditions: Option<String>,
    pub additional_information: Option<String>,
    pub cpv_code_and_name: String,

    #[serde(with = "time::serde::rfc3339")]
    pub create_date: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub tender_receipt_deadline: OffsetDateTime,
}
