use crate::esi::get_from_public_esi;
use crate::model::corporation::Corporation;

pub async fn get_corporation(corporation_id: i32) -> Result<Corporation, reqwest::Error> {
    let url = format!(
        "https://esi.evetech.net/latest/corporations/{}/?datasource=tranquility",
        corporation_id
    );

    get_from_public_esi::<Corporation>(&url).await
}
