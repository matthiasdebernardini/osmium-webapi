use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Entry {
    pub pubkey: String,
    pub backup: String,
    pub ln_invoice: String,
}
