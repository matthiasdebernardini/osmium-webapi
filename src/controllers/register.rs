use axum::extract::State;
use axum::Json;
use lightning_invoice::Invoice;
use lnbits_rust::{api::invoice::CreateInvoiceParams, LNBitsClient};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    error::AppError,
    models::{self, auth::Claims},
    utils::get_timestamp_one_year_from_now,
};

#[axum::debug_handler]
pub async fn register(
    State(pool): State<PgPool>,
    Json(e): Json<models::entry::Entry>,
) -> Result<Json<Value>, AppError> {
    if e.pubkey.is_empty() || e.backup.is_empty() || e.ln_invoice.is_empty() {
        return Err(AppError::MissingData);
    }

    let entry = sqlx::query_as::<_, models::entry::Entry>(
        "SELECT pubkey, backup, ln_invoice FROM entries where pubkey = $1",
    )
    .bind(&e.pubkey)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(_) = entry {
        return Err(AppError::EntryAlreadyExists);
    }
    let client = LNBitsClient::new(
        "a5504d243e5841d7afda898d49ad1edb",
        "1291ce79eeb84b0fb3f9357c543f4924",
        "0dd665d5eb6d465f9b17f6dd165f2021",
        "http://legend.lnbits.com",
        None,
    )
    .unwrap();

    let wallet_details = client.get_wallet_details().await.unwrap();

    println!("wallet_details: {:?}", wallet_details);

    let invoice = str::parse::<Invoice>(e.ln_invoice.as_str()).expect("msg");
    let invoice_timeout = invoice.expiry_time().as_secs();
    dbg!(invoice_timeout);
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

    println!("invoice: {invoice:?}");

    let result = sqlx::query("INSERT INTO entries (pubkey, backup, ln_invoice) values ($1, $2)")
        .bind(&e.pubkey)
        .bind(e.backup)
        .bind(e.ln_invoice)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    match result.rows_affected() {
        1 => Ok(Json(json!({ "pubkey": e.pubkey }))),
        _ => Err(AppError::InternalServerError),
    }
}
