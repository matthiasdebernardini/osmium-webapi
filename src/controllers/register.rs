use axum::extract::State;
use axum::Json;
use chrono::prelude::*;
use lightning_invoice::Invoice;
use lnbits_rust::{api::invoice::CreateInvoiceParams, LNBitsClient};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::collections::HashMap;
use std::process::exit;

use crate::{
    error::AppError,
    models::{self, auth::Claims},
    utils::get_timestamp_one_year_from_now,
};

pub async fn register(
    State(pool): State<PgPool>,
    State(client): State<LNBitsClient>,
    State(invoices): State<HashMap<String, String>>,
    Json(e): Json<models::entry::Entry>,
) -> Result<Json<Value>, AppError> {
    if e.pubkey.is_empty() || e.backup.is_empty() || e.ln_invoice.is_empty() {
        return Err(AppError::MissingData);
    }

    let wallet_details = match client.get_wallet_details().await {
        Ok(wallet_details) => wallet_details,
        Err(_) => return Ok(axum::Json(serde_json::json!(""))),
    };
    println!("wallet_details: {:?}", wallet_details);

    let invoice = str::parse::<Invoice>(e.ln_invoice.as_str()).expect("msg");
    let invoice_timeout = invoice.expiry_time().as_secs();
    dbg!(invoice_timeout);
    let timestamp = invoice.duration_since_epoch().as_secs() as i64;
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    // Format the datetime how you want
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

    // Print the newly formatted date and time
    println!("{}", newdate);
    let invoice = invoice.payment_hash().to_string();
    let mut ten_min_counter = 0;

    while !client.is_invoice_paid(&invoice).await.unwrap() {
        println!("Waiting for payment");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        // wait 10m for payment
        // TODO figure out CLTV for lnbits then use that instead of 10m
        match ten_min_counter {
            invoice_timeout => return Err(AppError::InvoiceNotPaid),
            _ => ten_min_counter += 1,
        }
    }

    match sqlx::query!(
        // "UPDATE entries SET ln_invoice = $1, backup = $3 WHERE pubkey = $2",
        "INSERT INTO entries (pubkey, ln_invoice, backup) VALUES ($1, $2, $3)",
        &e.pubkey,
        &invoice,
        &e.backup
    )
    .execute(&pool)
    .await
    {
        Ok(_) => Ok(Json(json!({ "pubkey": e.pubkey }))),
        Err(_) => Err(AppError::InternalServerError),
    }
}
