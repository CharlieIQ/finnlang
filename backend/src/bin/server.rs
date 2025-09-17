use axum::{routing::post, Json, Router};
use finnlang::run_finn_code;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tokio::time::timeout;

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
    // Add timeout to prevent infinite loops
    let result = timeout(Duration::from_secs(5), async {
        run_finn_code(&payload.code)
    }).await;
    
    match result {
        Ok(Ok(output)) => Json(RunResponse {
            output,
            error: None,
            success: true,
        }),
        Ok(Err(error)) => Json(RunResponse {
            output: String::new(),
            error: Some(error.to_string()),
            success: false,
        }),
        Err(_) => Json(RunResponse {
            output: String::new(),
            error: Some("Code execution timed out (5 seconds)".to_string()),
            success: false,
        }),
    }
}

#[tokio::main]
async fn main() {
    // Get port from environment variable (Render provides this) or default to 3000
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Set up CORS to allow requests from anywhere
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins (for dev; restrict in production)
        .allow_methods(Any) // Allow all methods: POST, OPTIONS, etc.
        .allow_headers(Any); // Allow all headers

    let app = Router::new().route("/run", post(run_code)).layer(cors); // Attach CORS middleware
    
    // Bind to all interfaces (0.0.0.0) so Render can access it
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
