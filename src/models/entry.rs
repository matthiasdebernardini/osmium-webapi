use serde::{Deserialize, Serialize};

#[derive(Deserialize, sqlx::FromRow)]
pub struct Entry {
    pub pubkey: String,
    pub backup: String,
}
