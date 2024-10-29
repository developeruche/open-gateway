//! This module holds the handlers for brand base entities.
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chronicle_primitives::{
    db::{
        entities::{
            brand::{
                get_total_count_brands, query_all_brands_paginated, query_brand_by_id,
                query_brand_by_name,
            },
            reward::query_all_rewards_owned_by_a_brand,
        },
        raw_chronicle_event::create_db_instance,
    },
    entity_record::BrandRecord,
    handler_payload::{BrandAndItRewards, PaginatedData, TotalDetail},
};
use serde::Deserialize;

use crate::{utils::AppError, AppState};

#[derive(Deserialize)]
pub struct GetAllBrandsFilter {
    pub page: i64,
    pub limit: i64,
}

/// This function is used to query  brand by name.
/// this `name` is specified in the query parameter.
pub async fn get_brand_by_name(
    State(state): State<Arc<AppState>>,
    Path(brand_name): Path<String>,
) -> Result<Json<BrandRecord>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let brand_record = query_brand_by_name(brand_name, &mut db_client).await?;

    Ok(Json(brand_record))
}

/// This function is used to get brands by id
/// this `id` is specified in the query parameter.
/// it is the protocol brand id
pub async fn get_brand_by_id(
    State(state): State<Arc<AppState>>,
    Path(brand_id): Path<String>,
) -> Result<Json<BrandRecord>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let brand_record = query_brand_by_id(brand_id, &mut db_client).await?;

    Ok(Json(brand_record))
}

/// This function is used to get all brands
/// this query  is in the paginated form
pub async fn get_all_brands_paginated(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<GetAllBrandsFilter>,
) -> Result<Json<PaginatedData<BrandRecord>>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let brand_records =
        query_all_brands_paginated(&mut db_client, filter.page, filter.limit).await?;

    let total_items = get_total_count_brands(&mut db_client).await?;
    let page_size = filter.limit;
    let total_page = (total_items + page_size - 1) / page_size;
    let current_page = filter.page;

    let paginated_format = PaginatedData::new(
        brand_records,
        total_page,
        total_items,
        current_page,
        page_size,
    );

    Ok(Json(paginated_format))
}

pub async fn get_brand_count(
    State(state): State<Arc<AppState>>,
) -> Result<Json<TotalDetail>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let brand_count = get_total_count_brands(&mut db_client).await?;

    let total_detail = TotalDetail {
        total: brand_count.to_string(),
    };

    Ok(Json(total_detail))
}

pub async fn get_all_rewards_owned_by_a_brand(
    State(state): State<Arc<AppState>>,
    Path(brand_id): Path<String>,
) -> Result<Json<BrandAndItRewards>, AppError> {
    let mut db_client = create_db_instance(&state.db_url).await?;
    let brand_record = query_brand_by_id(brand_id.clone(), &mut db_client).await?;
    let rewards = query_all_rewards_owned_by_a_brand(brand_id, &mut db_client).await?;

    let brand_and_it_rewards = BrandAndItRewards {
        brand_detail: brand_record,
        rewards,
    };

    Ok(Json(brand_and_it_rewards))
}
