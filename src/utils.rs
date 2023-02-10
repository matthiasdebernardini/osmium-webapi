use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::extract::FromRequest;
use axum::http::Request;
use axum::{
    async_trait,
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};

use crate::{error::AppError, models::auth::Claims};

pub fn get_timestamp_one_year_from_now() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let one_year_from_now = since_the_epoch + Duration::from_secs(60 * 60 * 24 * 365);
    one_year_from_now.as_secs()
}
