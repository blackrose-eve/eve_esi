use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use eve_esi::{character::get_character, corporation::get_corporation, initialize_eve_esi};
use serde::Deserialize;

#[derive(Deserialize)]
struct GetByIdParams {
    id: i32,
}

#[tokio::main]
async fn main() {
    let application_name = "Black Rose EVE ESI Example";
    let application_email = "example@example.com";

    initialize_eve_esi(application_name.to_string(), application_email.to_string());

    let app = Router::new()
        .route("/character", get(get_esi_character))
        .route("/corporation", get(get_esi_corporation));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("Test character API at http://localhost:8000/character?id=2114794365");
    axum::serve(listener, app).await.unwrap();
}

async fn get_esi_character(params: Query<GetByIdParams>) -> Response {
    match get_character(params.0.id).await {
        Ok(character) => (StatusCode::OK, Json(character)).into_response(),
        Err(error) => {
            let status_code = StatusCode::from_u16(error.status().unwrap().into()).unwrap();

            (status_code, Json(error.to_string())).into_response()
        }
    }
}

async fn get_esi_corporation(params: Query<GetByIdParams>) -> Response {
    match get_corporation(params.0.id).await {
        Ok(corporation) => (StatusCode::OK, Json(corporation)).into_response(),
        Err(error) => {
            let status_code = StatusCode::from_u16(error.status().unwrap().into()).unwrap();

            (status_code, Json(error.to_string())).into_response()
        }
    }
}
