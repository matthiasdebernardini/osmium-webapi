
use anyhow::Result;
use axum::{Extension, Json, RequestPartsExt};
use jsonwebtoken::{encode, Header};
use rand::Rng;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::{thread, time};
use axum::debug_handler;
use axum::extract::{FromRequestParts, Path, State};
use axum::http::request::Parts;

use crate::{
    error::AppError,
    models::{self, auth::Claims},
    utils::get_timestamp_8_hours_from_now,
    KEYS,
};
use crate::models::entry::Entry;





#[debug_handler]
pub async fn recover_backup(
    Path(pubkey): Path<String>,
    State(pool): State<PgPool>,
    // Json(entry): Json<models::entry::Entry>,
) -> axum::Json<serde_json::Value> {
    // check if email or password is a blank string
    dbg!(pubkey.clone());
    if pubkey.is_empty() {
        let mut rng = rand::thread_rng();
        let millis = rng.gen_range(0..25);
        thread::sleep(time::Duration::from_millis(millis));

        return axum::Json(serde_json::json!(""));
    }

    // get the user for the email from database
    let backup =
        sqlx::query_as::<_, models::recover::Recover>("SELECT backup FROM entries where pubkey = $1")
            .bind(&pubkey)
            .fetch_optional(&pool)
            .await
            .map_err(|err| {
                dbg!(err);
                AppError::InternalServerError
            });
    let backup = match backup {
        Ok(backup) => backup,
        Err(_) => return axum::Json(serde_json::json!("")),
    };
    let backup = match backup {
        Some(e) => e.pubkey,
        None => "".to_string(),
    };

    axum::Json(serde_json::json!({
        "backup": backup,
    }))
}