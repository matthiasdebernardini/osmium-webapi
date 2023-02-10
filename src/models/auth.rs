use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub pubkey: String,
    pub exp: u64,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Receipts {
    pub ln_invoice: String,
    pub ln_preimage: String,
    pub pubkey: String,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
