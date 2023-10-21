use crate::osrs;
use mongodb::bson::{DateTime, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UsernameEntry {
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountEntry {
    pub account_hash: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatEntry {
    pub timestamp: DateTime,
    pub display_name: String,
    pub stats: osrs::Hiscore,
}
