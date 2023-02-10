use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct Recover(pub String);
