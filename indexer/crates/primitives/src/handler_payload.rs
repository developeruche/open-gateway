//! This file holds the payload types that would be sent to via the handler.
//! these types needs to be serializable and deserializable in JSON format.
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::entity_record::{BrandRecord, RewardRecord};

#[derive(Serialize, Deserialize, Debug, Clone, Default, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct GetRewardData {
    pub reward_name: String,
    pub circulating_supply: String,
    pub number_of_holders: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct GetRedemptionTransactionData {
    pub source_token: String,
    pub dest_token: String,
    pub source_amount: String,
    pub dest_amount: String,
    pub user_address: String,
    pub onchain_tx_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct TotalDetail {
    pub total: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrandAndItRewards {
    pub brand_detail: BrandRecord,
    pub rewards: Vec<RewardRecord>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedData<T> {
    pub data: Vec<T>,
    pub total_page: i64,
    pub total_items: i64,
    pub current_page: i64,
    pub page_size: i64,
}

impl<T> PaginatedData<T> {
    pub fn new(
        data: Vec<T>,
        total_page: i64,
        total_items: i64,
        current_page: i64,
        page_size: i64,
    ) -> Self {
        Self {
            data,
            total_page,
            total_items,
            current_page,
            page_size,
        }
    }
}
