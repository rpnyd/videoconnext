use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing_subscriber::EnvFilter;

// Saare modules import karo
mod state;
mod models;
mod auth;
mod chat;
mod location;
mod files;

use state::AppState;

// =====================================================================
// MAIN — server yahan shuru hota hai
// =====================================================================

#[tokio::main]
async fn main() {
    // Logging setup — RUST_LOG=info cargo run se level control kar sakte ho
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    // Uploads folder banao agar nahi hai
    tokio::fs::create_dir_all("uploads")
        .await
        .expect("uploads folder nahi ban saka");

    // Shared app state banao
    let app_state = AppState::new();

    // ----------------------------------------------------------------
    // ROUTES
    // ----------------------------------------------------------------
    let app = Router::new()

        // --- Auth ---
        .route("/signup", post(auth::handlers::signup))
        .route("/login",  post(auth::handlers::login))

        // --- WebSocket ---
        // Connect: ws://localhost:3000/ws?token=YOUR_JWT
        .route("/ws", get(chat::ws::ws_handler))

        // --- File Upload ---
        .route("/upload", post(files::handlers::upload_file))

        // --- Uploaded files serve karo (photos/files access ke liye) ---
        .nest_service("/files", ServeDir::new("uploads"))

        // --- Static frontend files serve karo ---
        .nest_service("/", ServeDir::new("static"))

        // --- State inject karo ---
        .with_state(app_state)

        // --- CORS: Browser se requests allow karo ---
        .layer(CorsLayer::permissive());

    // ----------------------------------------------------------------
    // SERVER START
    // ----------------------------------------------------------------
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Port 3000 already use mein hai, ya permission nahi");

    tracing::info!("===========================================");
    tracing::info!("  VideoConnect server chal raha hai!");
    tracing::info!("  http://localhost:3000");
    tracing::info!("===========================================");

    axum::serve(listener, app)
        .await
        .expect("Server crash ho gaya");
}
