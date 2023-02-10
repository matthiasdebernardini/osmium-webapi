use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// import module
mod controllers;
mod error;
mod models;
mod utils;

#[derive(FromRef, Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable, such as DATABASE_URL=postgresql://postgres:password@0.0.0.0:5432 for example");
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "osmium_webapi=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new().allow_origin(Any);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to connect to database");

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS entries (
  pubkey text,
  backup text,
  ln_invoice text
);"#,
    )
    .execute(&pool)
    .await
    .expect("unable to create table");
    info!("Connected to database");

    let state = AppState { pool };
    let app = Router::new()
        .route("/", get(controllers::info::route_info))
        .route("/register", post(controllers::register::register))
        .route(
            "/recover/:pubkey",
            get(controllers::recover::recover_backup),
        )
        .route("/payment/:pubkey", get(controllers::payment::payment))
        .layer(cors)
        .with_state(state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
