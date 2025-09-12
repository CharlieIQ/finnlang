use axum::{routing::post, Json, Router};
use finnlang::run_finn_code;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct RunRequest {
    code: String,
}

#[derive(Serialize)]
struct RunResponse {
    output: String,
    error: Option<String>,
    success: bool,
}

async fn run_code(Json(payload): Json<RunRequest>) -> Json<RunResponse> {
    match run_finn_code(&payload.code) {
        Ok(output) => Json(RunResponse {
            output,
            error: None,
            success: true,
        }),
        Err(error) => Json(RunResponse {
            output: String::new(),
            error: Some(error.to_string()),
            success: false,
        }),
    }
}

#[tokio::main]
async fn main() {
    // Set up CORS to allow requests from anywhere
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins (for dev; restrict in production)
        .allow_methods(Any) // Allow all methods: POST, OPTIONS, etc.
        .allow_headers(Any); // Allow all headers

    let app = Router::new().route("/run", post(run_code)).layer(cors); // Attach CORS middleware
    // You can change the address and port as needed
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
