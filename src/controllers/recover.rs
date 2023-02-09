use anyhow::Result;
use axum::{Extension, Json, RequestPartsExt};
use jsonwebtoken::{encode, Header};
use rand::Rng;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::{thread, time};
use axum::debug_handler;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::{
    error::AppError,
    models::{self, auth::Claims},
    utils::get_timestamp_8_hours_from_now,
    KEYS,
};
use crate::models::entry::Entry;

// #[async_trait]
// impl<S> FromRequestParts<S> for Entry
//     where
//         S: Send + Sync,
// {
//     type Rejection = AppError;
//
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         if let Some(user_agent) = parts.extract(). .extensions.get() .headers.get(USER_AGENT) {
//             Ok(ExtractUserAgent(user_agent.clone()))
//         } else {
//             Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
//         }
//     }
// }



#[debug_handler]
pub async fn recover_backup(
    Json(entry): Json<models::entry::Entry>,
    Extension(pool): Extension<PgPool>,
) -> axum::Json<serde_json::Value> {
    // check if email or password is a blank string
    if entry.pubkey.is_empty() {
        let mut rng = rand::thread_rng();
        let millis = rng.gen_range(0..25);
        thread::sleep(time::Duration::from_millis(millis));

        return axum::Json(serde_json::json!(""));
    }

    // get the user for the email from database
    let backup =
        sqlx::query_as::<_, models::entry::Entry>("SELECT backup FROM entries where pubkey = $1")
            .bind(&entry.pubkey)
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
        Some(e) => e.backup,
        None => "".to_string(),
    };

    axum::Json(serde_json::json!({
        "backup": backup,
    }))
}
