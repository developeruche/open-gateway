//! Handlers for reward based entities.
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chronicle_primitives::{
    db::{
        entities::reward::{
            get_total_rewards_count, query_all_rewards_paginated, query_reward_by_brand_id,
            query_reward_by_requestor_address,
        },
        raw_chronicle_event::create_db_instance,
    },
    entity_record::RewardRecord,
    handler_payload::{PaginatedData, TotalDetail},
};
use serde::Deserialize;

use crate::{utils::AppError, AppState};

#[derive(Deserialize)]
pub struct GetAllRewardsFilter {
    pub page: i64,
    pub limit: i64,
}

/// This function is used to get all rewards in paginated form
pub async fn get_all_rewards(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<GetAllRewardsFilter>,
) -> Result<Json<PaginatedData<RewardRecord>>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let reward_record =
        query_all_rewards_paginated(&mut db_client, filter.page, filter.limit).await?;

    let total_items = get_total_rewards_count(&mut db_client).await?;
    let page_size = filter.limit;
    let total_page = (total_items + page_size - 1) / page_size;
    let current_page = filter.page;

    let paginated_data = PaginatedData::new(
        reward_record,
        total_page,
        total_items,
        current_page,
        page_size,
    );

    Ok(Json(paginated_data))
}

/// This function is used to get rewards by brand_id
/// this `brand_id` is specified in the query parameter.
pub async fn get_reward_by_brand_id(
    State(state): State<Arc<AppState>>,
    Path(brand_id): Path<String>,
) -> Result<Json<RewardRecord>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let reward_record = query_reward_by_brand_id(brand_id, &mut db_client).await?;

    Ok(Json(reward_record))
}

/// This function is used to get rewards by reward_address
/// this `reward_address` is specified in the query parameter.
pub async fn get_reward_by_reward_address(
    State(state): State<Arc<AppState>>,
    Path(reward_address): Path<String>,
) -> Result<Json<RewardRecord>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let reward_record = query_reward_by_requestor_address(reward_address, &mut db_client).await?;

    Ok(Json(reward_record))
}

/// This function is used to get rewards by reward_address
/// this `reward_address` is specified in the query parameter.

pub async fn get_reward_count(
    State(state): State<Arc<AppState>>,
) -> Result<Json<TotalDetail>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let reward_count = get_total_rewards_count(&mut db_client).await?;

    let total_detail = TotalDetail {
        total: reward_count.to_string(),
    };
    Ok(Json(total_detail))
}
