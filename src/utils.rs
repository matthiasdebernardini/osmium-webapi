use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::extract::FromRequest;
use axum::http::Request;
use axum::{
    async_trait,
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};

use crate::{error::AppError, models::auth::Claims, KEYS};

// get 8 hours timestamp for jwt expiry
// todo change this to one year
pub fn get_timestamp_8_hours_from_now() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let eighthoursfromnow = since_the_epoch + Duration::from_secs(28800);
    eighthoursfromnow.as_secs()
}

// verify token and extract data from it (a kind of middleware), whenever you try to extract claims in the handle it will first run this code
#[async_trait]
impl<S, B> FromRequest<S, B> for Claims
where
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req, state)
                .await
                .map_err(|_| AppError::InvalidToken)?;
        let data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AppError::InvalidToken)?;
        Ok(data.claims)
    }
}
