use crate::esi::get_from_public_esi;
use crate::model::alliance::Alliance;

pub async fn get_alliance(alliance_id: i32) -> Result<Alliance, reqwest::Error> {
    let url = format!(
        "https://esi.evetech.net/latest/alliances/{}/?datasource=tranquility",
        alliance_id
    );

    get_from_public_esi::<Alliance>(&url).await
}
