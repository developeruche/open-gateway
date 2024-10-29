//! This role of this module is to take database data and convert it to a rust type
//! that can easily be serialized and deserialized for other chornicle component to make
//! use of.
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PoolRecord {
    pub pool_address: String,
    pub reward_token: String,
    pub me_token: String,
    pub current_amount_of_reward_tokens: String,
    pub current_amount_of_me_tokens: String,
    pub r_optimal: String,
    pub r: String,
    pub create_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrandRecord {
    pub brand_name: String,
    pub main_account: String,
    pub online_presence: String,
    pub brand_protocol_id: String,
    pub onboarding_manager: String,
    pub create_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RedepmtionRecord {
    pub source_token: String,
    pub dest_token: String,
    pub source_amount: String,
    pub dest_amount: String,
    pub user_address: String,
    pub onchain_tx_hash: String,
    pub redeemed_at: String,
    pub create_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RewardRecord {
    pub brand_id: String,
    pub reward_address: String,
    pub requestor_address: String,
    pub initial_supply: String,
    pub timestamp: String,
    pub create_at: NaiveDateTime,
}
