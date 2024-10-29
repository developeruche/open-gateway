//! This file holds transtion related handlers.
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chronicle_primitives::{
    db::{
        entities::redemption::{
            get_total_redemption_on_reward_count, get_total_redemption_on_user_count,
            get_total_redemptions_count, query_all_redemptions_paginated,
            query_all_redemptions_paginated_by_reward_address,
            query_all_redemptions_paginated_by_user_address, query_redemption_by_onchain_tx_hash,
        },
        raw_chronicle_event::create_db_instance,
    },
    entity_record::RedepmtionRecord,
    handler_payload::{PaginatedData, TotalDetail},
};
use serde::Deserialize;

use crate::{utils::AppError, AppState};

#[derive(Deserialize)]
pub struct RedeptionByIdFilter {
    pub page: i64,
    pub limit: i64,
}

pub async fn get_all_redemptions(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<RedeptionByIdFilter>,
) -> Result<Json<PaginatedData<RedepmtionRecord>>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let redemption_records =
        query_all_redemptions_paginated(&mut db_client, filter.page, filter.limit).await?;

    let total_items = get_total_redemptions_count(&mut db_client).await?;
    let page_size = filter.limit;
    let total_page = (total_items + page_size - 1) / page_size;
    let current_page = filter.page;

    let paginated_data = PaginatedData::new(
        redemption_records,
        total_page,
        total_items,
        current_page,
        page_size,
    );

    Ok(Json(paginated_data))
}

pub async fn get_all_redemptions_by_reward_id(
    State(state): State<Arc<AppState>>,
    Path(reward_address): Path<String>,
    Query(filter): Query<RedeptionByIdFilter>,
) -> Result<Json<PaginatedData<RedepmtionRecord>>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let redemption_records = query_all_redemptions_paginated_by_reward_address(
        &mut db_client,
        filter.page,
        filter.limit,
        reward_address.clone(),
    )
    .await?;

    let total_items = get_total_redemption_on_reward_count(&mut db_client, reward_address).await?;
    let page_size = filter.limit;
    let total_page = (total_items + page_size - 1) / page_size;
    let current_page = filter.page;

    let paginated_data = PaginatedData::new(
        redemption_records,
        total_page,
        total_items,
        current_page,
        page_size,
    );

    Ok(Json(paginated_data))
}

pub async fn get_redeption_count(
    State(state): State<Arc<AppState>>,
) -> Result<Json<TotalDetail>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let redemption_count = get_total_redemptions_count(&mut db_client).await?;

    let total_detail = TotalDetail {
        total: redemption_count.to_string(),
    };

    Ok(Json(total_detail))
}

pub async fn get_all_redemption_by_user_address(
    State(state): State<Arc<AppState>>,
    Path(user_address): Path<String>,
    Query(filter): Query<RedeptionByIdFilter>,
) -> Result<Json<PaginatedData<RedepmtionRecord>>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let redemption_records = query_all_redemptions_paginated_by_user_address(
        &mut db_client,
        filter.page,
        filter.limit,
        user_address.clone(),
    )
    .await?;

    let total_items = get_total_redemption_on_user_count(&mut db_client, user_address).await?;
    let page_size = filter.limit;
    let total_page = (total_items + page_size - 1) / page_size;
    let current_page = filter.page;

    let paginated_data = PaginatedData::new(
        redemption_records,
        total_page,
        total_items,
        current_page,
        page_size,
    );

    Ok(Json(paginated_data))
}

pub async fn get_redemption_by_onchain_tx_hash(
    State(state): State<Arc<AppState>>,
    Path(onchain_tx_hash): Path<String>,
) -> Result<Json<RedepmtionRecord>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let redemption_record =
        query_redemption_by_onchain_tx_hash(&mut db_client, onchain_tx_hash).await?;

    Ok(Json(redemption_record))
}
