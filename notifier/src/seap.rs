use crate::advertisment::Advertisment;
use anyhow::{Error, Result};
use serde_json::Value;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub async fn get_last_adv_ids(
    client: reqwest::Client,
    n: usize,
    time: OffsetDateTime,
) -> Result<Vec<u32>> {
    let resp = client
        .post("http://e-licitatie.ro/api-pub/AdvNoticeCommon/GetAdvNoticeList/")
        .header("Referer", "http://e-licitatie.ro/pub/adv-notices/list/1")
        .header("Content-Type", "application/json")
        .body(format!(
            "
        {{
            \"pageSize\": {},
            \"publicationDateStart\": \"{}\",
            \"pageIndex\": 0
        }}
        ",
            n,
            time.format(&Rfc3339)?
        ))
        .send()
        .await?
        .text()
        .await?;

    let v: Value = serde_json::from_str(&resp)?;
    let ads = v
        .get("items")
        .ok_or_else(|| Error::msg("The json doesn't have an 'items' key."))?;
    Ok(ads
        .as_array()
        .ok_or_else(|| Error::msg("The object couldn't be converted into an array."))?
        .iter()
        .map(|val| val.get("advNoticeId").unwrap().as_u64().unwrap() as u32)
        .collect::<Vec<u32>>())
}

pub async fn get_adv(client: reqwest::Client, id: u32) -> Result<Advertisment> {
    let resp = client
        .get(format!(
            "http://e-licitatie.ro/api-pub/PUBLICAdvNotice/getForView/{}",
            id
        ))
        .header("Content-Type", "application/json")
        .header(
            "Referer",
            "http://e-licitatie.ro/pub/notices/adv-notices/view/100449529",
        )
        .send()
        .await?
        .text()
        .await?;

    let adv: Advertisment = serde_json::from_str(&resp)?;

    Ok(adv)
}
