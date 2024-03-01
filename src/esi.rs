use serde::{de::DeserializeOwned, Serialize};

use crate::{APP_EMAIL, APP_NAME};

pub async fn get_from_public_esi<T: DeserializeOwned>(url: &str) -> Result<T, reqwest::Error> {
    let user_agent = {
        let app_name = APP_NAME.lock().unwrap();
        let app_email = APP_EMAIL.lock().unwrap();
        format!("{} ({})", *app_name, *app_email)
    };

    let req = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, user_agent)
        .send()
        .await?;

    let result: T = req.json().await?;

    Ok(result)
}

pub async fn post_to_public_esi<T: DeserializeOwned, U: Serialize + ?Sized>(
    url: &str,
    data: &U,
) -> Result<T, reqwest::Error> {
    let user_agent = {
        let app_name = APP_NAME.lock().unwrap();
        let app_email = APP_EMAIL.lock().unwrap();
        format!("{} ({})", *app_name, *app_email)
    };

    let req = reqwest::Client::new()
        .post(url)
        .header(reqwest::header::USER_AGENT, user_agent)
        .json(data)
        .send()
        .await?;

    let result: T = req.json().await?;

    Ok(result)
}
