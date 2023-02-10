use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InvalidToken,
    MissingData,
    TokenCreation,
    InternalServerError,
    EntryAlreadyExists,
    InvoiceNotPaid,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occured",
            ),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "invalid token"),
            Self::MissingData => (StatusCode::BAD_REQUEST, "missing data"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "failed to create token"),
            Self::EntryAlreadyExists => (StatusCode::BAD_REQUEST, "Entry already exists"),
            Self::InvoiceNotPaid => (
                StatusCode::PAYMENT_REQUIRED,
                "Lightning invoice is not paid",
            ),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
