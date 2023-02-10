use serde::{Deserialize, Serialize};

#[derive(Deserialize, sqlx::FromRow)]
pub struct Recover{
    pub pubkey: String,
}
