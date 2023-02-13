use axum::debug_handler;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use lnbits_rust::{api::invoice::CreateInvoiceParams, LNBitsClient};
use rand::Rng;
use sqlx::PgPool;
use std::collections::HashMap;
use std::{thread, time};
use tracing::debug;

use crate::error::AppError;
use crate::models::entry::Entry;
use crate::models::recover::PubKey;
use crate::SharedState;

pub async fn payment(
    Path(pubkey): Path<PubKey>,
    State(state): State<SharedState>,

    // State(pool): State<PgPool>,
    // State(client): State<LNBitsClient>,
    // State(&mut invoices): State<HashMap<String, String>>,
) -> axum::Json<serde_json::Value> {
    let db = &state.read().unwrap().db.pool;

    dbg!(pubkey.clone());
    eprintln!("pubkey.0.is_empty() {}", pubkey.0.is_empty());
    eprintln!("invoices.contains_key(&pubkey.0) {}", invoices.contains_key(&pubkey.0));
    if pubkey.0.is_empty() || invoices.contains_key(&pubkey.0){
        let mut rng = rand::thread_rng();
        let millis = rng.gen_range(0..25);
        thread::sleep(time::Duration::from_millis(millis));

        return axum::Json(serde_json::json!(""));
    }


    //     .map(|invoice| {
    //     axum::Json(serde_json::json!({
    //         "invoice": invoice.clone(),
    //     }))
    // }).unwrap_or_else(|| {
    //     axum::Json(serde_json::json!(""))
    // })

    let wallet_details = match client.get_wallet_details().await {
        Ok(wallet_details) => wallet_details,
        Err(_) => return axum::Json(serde_json::json!("")),
    };

    debug!(wallet_details = ?wallet_details, "wallet_details");

    let invoice = match client
        .create_invoice(&CreateInvoiceParams {
            amount: 1,
            unit: "sat".to_string(),
            memo: None,
            expiry: Some(10000),
            webhook: None,
            internal: None,
        })
        .await
    {
        Ok(invoice) => invoice.payment_request,
        Err(_) => return axum::Json(serde_json::json!("")),
    };

    // debug!(invoice = ?invoice, "invoice");
    println!("invoice: {}", invoice);

    let a = match invoices.insert(pubkey.0.clone(), invoice.clone()) {
        None => axum::Json(serde_json::json!({
        "pubkey": pubkey.clone(),
        "ln_invoice": invoice,
        })),
        Some(_) => axum::Json(serde_json::json!("")),
    };
    for (key, value) in &*invoices {
        println!("{} / {}", key, value);
    }
    invoices.clear();
    a
    // };
    // match sqlx::query!(
    //     "UPDATE entries SET ln_invoice = $1 WHERE pubkey = $2",
    //     &invoice,
    //     &pubkey.0
    // )
    //     .execute(&pool)
    //     .await {
    //     Ok(_) => {
    // axum::Json(serde_json::json!({
    //     "pubkey": pubkey.clone(),
    //     "ln_invoice": invoice,
    // }))},
    //     Err(_) => axum::Json(serde_json::json!("")),
    // }
}
// check if pubkey already exists

// let entry = sqlx::query_as::<_, Entry>("SELECT * FROM entries where pubkey = $1")
//     .bind(&pubkey.0)
//     .fetch_optional(&pool)
//     .await;

// match entry {
//     Err(_) => return axum::Json(serde_json::json!("")),
//     Ok(o) => match o {
//         Some(e) => {
//             // update invoice
//             let _ = sqlx::query!(
//                 "UPDATE entries SET invoice = $1 WHERE pubkey = $2",
//                 &invoice,
//                 &pubkey.0
//             )
//             .execute(&pool)
//             .await;
//             return axum::Json(serde_json::json!(""))
//         }
//         None => axum::Json(serde_json::json!({
//             "pubkey": pubkey.clone(),
//             "ln_invoice": invoice,
//         })),
//     },
// }
