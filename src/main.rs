use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// import module
mod controllers;
mod error;
mod models;
mod utils;
use lnbits_rust::{api::invoice::CreateInvoiceParams, LNBitsClient};
use tokio::sync::RwLock;

#[derive(FromRef, Clone, Default)]
struct AppState {
    pool: PgPool,
    client: LNBitsClient,
    // todo change to pubkey and invoice
    invoices: std::collections::HashMap<String, String>,
}

type SharedState = Arc<RwLock<AppState>>;

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
	pubkey  text PRIMARY KEY,
  backup text,
  ln_invoice text
);"#,
    )
    .execute(&pool)
    .await
    .expect("unable to create table");
    info!("Connected to database");

    // https://legend.lnbits.com/wallet?usr=821dfb899bd0488284642a65b84d7d39&wal=a5504d243e5841d7afda898d49ad1edb
    let client = LNBitsClient::new(
        "a5504d243e5841d7afda898d49ad1edb",
        "1291ce79eeb84b0fb3f9357c543f4924",
        "0dd665d5eb6d465f9b17f6dd165f2021",
        "http://legend.lnbits.com",
        None,
    )
    .unwrap();

    let mut invoices: HashMap<String, String> = std::collections::HashMap::new();
    let shared_state = Arc::new(RwLock::new(AppState {
        pool,
        client,
        invoices,
    }));

    // let shared_state = AppState {
    //     pool,
    //     client,
    //     invoices,
    // };

    let app = Router::new()
        .route("/", get(controllers::info::route_info))
        .route("/payment/:pubkey", get(controllers::payment::payment))
        .route("/register", post(controllers::register::register))
        .route(
            "/recover/:pubkey",
            get(controllers::recover::recover_backup),
        )
        .layer(cors)
        .with_state(Arc::clone(&shared_state));

    // Todo add env variable for port
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
