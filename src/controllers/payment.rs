use axum::debug_handler;
use axum::extract::{Path, State};
use lnbits_rust::{api::invoice::CreateInvoiceParams, LNBitsClient};
use rand::Rng;
use sqlx::PgPool;
use std::{thread, time};
use axum::response::IntoResponse;

use crate::error::AppError;
use crate::models::recover::PubKey;

#[debug_handler]
pub async fn payment(
    Path(pubkey): Path<PubKey>,
    State(pool): State<PgPool>,
) -> axum::Json<serde_json::Value> {
    dbg!(pubkey.clone());
    if pubkey.0.is_empty() {
        let mut rng = rand::thread_rng();
        let millis = rng.gen_range(0..25);
        thread::sleep(time::Duration::from_millis(millis));

        return axum::Json(serde_json::json!(""));
    }
    // https://legend.lnbits.com/wallet?usr=821dfb899bd0488284642a65b84d7d39&wal=a5504d243e5841d7afda898d49ad1edb
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

    let invoice = client
        .create_invoice(&CreateInvoiceParams {
            amount: 1,
            unit: "sat".to_string(),
            memo: None,
            expiry: Some(10000),
            webhook: None,
            internal: None,
        })
        .await
        .unwrap();

    println!("invoice: {invoice:?}");

    // get the user for the email from database
    let existing_pubkey = sqlx::query_as::<_, PubKey>("SELECT pubkey FROM entries where pubkey = $1")
        .bind(&pubkey.0)
        .fetch_optional(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        });
        // .map(|pubkey| pubkey);
    // dbg!(existing_pubkey.expect("REASON").clone());
    let existing_pubkey = match existing_pubkey {
        Ok(pubkey) => pubkey,
        Err(_) => return axum::Json(serde_json::json!("")),
    };

    match  existing_pubkey {
        None => axum::Json(serde_json::json!({
            "pubkey": pubkey,
            "ln_invoice": invoice.payment_request,
        })),
        Some(_) => axum::Json(serde_json::json!("")),
    }
}
