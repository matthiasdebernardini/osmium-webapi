use axum::debug_handler;
use axum::extract::{Path, State};
use lightning_invoice::Invoice;
use lnbits_rust::LNBitsClient;
use rand::Rng;
use sqlx::PgPool;
use std::{thread, time};

use crate::error::AppError;
use crate::models::entry::Entry;
use crate::models::recover::PubKey;

pub async fn recover_backup(
    Path(pubkey): Path<PubKey>,
    State(client): State<LNBitsClient>,
    State(pool): State<PgPool>,
) -> axum::Json<serde_json::Value> {
    dbg!(pubkey.clone());
    if pubkey.0.is_empty() {
        let mut rng = rand::thread_rng();
        let millis = rng.gen_range(0..25);
        thread::sleep(time::Duration::from_millis(millis));

        return axum::Json(serde_json::json!(""));
    }

    // get the user for the email from database
    let entry = sqlx::query_as::<_, Entry>("SELECT * FROM entries where pubkey = $1")
        .bind(&pubkey.0)
        .fetch_optional(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        });

    println!("entry: {:?}", entry.clone());

    let entry = match entry {
        Ok(None) => return axum::Json(serde_json::json!("")),
        Ok(entry) => entry.expect("could not find entry"),
        Err(_) => return axum::Json(serde_json::json!("")),
    };
    let invoice = entry.ln_invoice.as_str();

    let invoice = str::parse::<Invoice>(invoice.clone()).expect("could not parse invoice");
    let invoice = invoice.payment_hash().to_string();

    while !client
        .is_invoice_paid(&invoice)
        .await
        .expect("could not check if invoice is paid")
    {
        println!("Waiting for payment");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    axum::Json(serde_json::json!({
        "backup": entry.backup.clone(),
    }))
}
