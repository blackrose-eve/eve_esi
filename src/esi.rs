use serde::{de::DeserializeOwned, Serialize};

use crate::USER_AGENT;

pub fn get_user_agent() -> String {
    let user_agent = USER_AGENT.lock().unwrap();
    user_agent.clone()
}

pub async fn get_from_public_esi<T: DeserializeOwned>(url: &str) -> Result<T, reqwest::Error> {
    let req = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, get_user_agent())
        .send()
        .await?;

    let result: T = req.json().await?;

    Ok(result)
}

pub async fn post_to_public_esi<T: DeserializeOwned, U: Serialize + ?Sized>(
    url: &str,
    data: &U,
) -> Result<T, reqwest::Error> {
    let req = reqwest::Client::new()
        .post(url)
        .header(reqwest::header::USER_AGENT, get_user_agent())
        .json(data)
        .send()
        .await?;

    let result: T = req.json().await?;

    Ok(result)
}
