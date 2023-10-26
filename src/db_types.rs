use crate::osrs;
use mongodb::bson::DateTime;
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StatEntry {
    pub timestamp: DateTime,
    pub display_name: String,
    pub stats: osrs::Hiscore,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TopPlayerEntry {
    pub display_name: String,
    pub league_points: u32,
}
