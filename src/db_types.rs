use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UsernameEntry {
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountEntry {
    pub account_hash: i64,
    pub display_name: String,
}
