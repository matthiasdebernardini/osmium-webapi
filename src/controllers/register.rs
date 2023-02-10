use axum::extract::State;
use axum::Json;
use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    error::AppError,
    models::{self, auth::Claims},
    utils::get_timestamp_8_hours_from_now,
    KEYS,
};

#[axum::debug_handler]
pub async fn register(
    State(pool): State<PgPool>,
    Json(e): Json<models::entry::Entry>,
) -> Result<Json<Value>, AppError> {
    // check if email or password is a blank string
    if e.pubkey.is_empty() || e.backup.is_empty() {
        return Err(AppError::MissingData);
    }

    // get the user for the email from database
    let entry = sqlx::query_as::<_, models::entry::Entry>(
        "SELECT pubkey, backup FROM entries where pubkey = $1",
    )
    .bind(&e.pubkey)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(_) = entry {
        //if a user with email already exits send error
        return Err(AppError::EntryAlreadyExits);
    }

    let result = sqlx::query("INSERT INTO entries (pubkey, backup) values ($1, $2)")
        .bind(&e.pubkey)
        .bind(e.backup)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    if result.rows_affected() < 1 {
        Err(AppError::InternalServerError)
    } else {
        let claims = Claims {
            pubkey: e.pubkey.to_owned(),
            exp: get_timestamp_8_hours_from_now(),
        };
        let token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| AppError::TokenCreation)?;
        // return bearer token
        Ok(Json(json!({ "access_token": token, "type": "Bearer" })))
    }
}
