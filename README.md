# EVE ESI

Black Rose's API wrapper for interaction with [EVE Online's ESI](https://esi.evetech.net/ui/).

## Implementation

1. Initialize ESI in your main function using the `initialize_eve_esi` function which will set the user agent, this includes your application name & contact email if CCP needs to reach out to you for any reason.
2. Use ESI routes as needed from there.

```rust
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
```

See the [axum](https://github.com/blackrose-eve/eve_esi/tree/main/examples/axum.rs) example.

To test out the example:
1. Run `cargo run --example axum`
2. Head to one of the URLs posted in your console, change the IDs to test out different characters/corporations

## Notes

- More ESI routes will be added as needed, feel free to submit pull requests to add any you may need.
- Only public ESI routes are available, private routes will be added at a later date.
