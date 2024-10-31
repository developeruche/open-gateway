//! This file holds all that handlers for pool related entities.
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chronicle_primitives::{
    db::{
        entities::pool::{
            get_pool_total_count, query_all_pools_paginated, query_pool_by_reward_address,
        },
        raw_chronicle_event::create_db_instance,
    },
    entity_record::PoolRecord,
    handler_payload::{PaginatedData, TotalDetail},
};
use serde::Deserialize;

use crate::{utils::AppError, AppState};

#[derive(Deserialize)]
pub struct GetAllPoolFilter {
    pub page: i64,
    pub limit: i64,
}

pub async fn get_all_pools(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<GetAllPoolFilter>,
) -> Result<Json<PaginatedData<PoolRecord>>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let pool_records = query_all_pools_paginated(&mut db_client, filter.page, filter.limit).await?;

    let total_items = get_pool_total_count(&mut db_client).await?;
    let page_size = filter.limit;
    let current_page = filter.page;
    let total_page = (total_items + page_size - 1) / page_size;

    let paginated_data = PaginatedData::new(
        pool_records,
        total_page,
        total_items,
        current_page,
        page_size,
    );

    Ok(Json(paginated_data))
}

pub async fn get_pool_by_reward_address(
    State(state): State<Arc<AppState>>,
    Path(reward_address): Path<String>,
) -> Result<Json<PoolRecord>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let pool_record = query_pool_by_reward_address(reward_address, &mut db_client).await?;

    Ok(Json(pool_record))
}

pub async fn get_pool_count(
    State(state): State<Arc<AppState>>,
) -> Result<Json<TotalDetail>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let pool_count = get_pool_total_count(&mut db_client).await?;

    let total_detail = TotalDetail {
        total: pool_count.to_string(),
    };

    Ok(Json(total_detail))
}
