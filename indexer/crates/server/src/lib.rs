pub mod handlers;
pub mod query;
pub mod utils;
use std::sync::Arc;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, ObjectType, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    http::Method,
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use chronicle_primitives::ServerConfig;
use handlers::{
    brand::{get_all_brands_paginated, get_brand_by_id, get_brand_by_name, get_brand_count},
    pool::{get_all_pools, get_pool_by_reward_address, get_pool_count},
    reward::{
        get_all_rewards, get_reward_by_brand_id, get_reward_by_reward_address, get_reward_count,
    },
    transaction::{
        get_all_redemption_by_user_address, get_all_redemptions, get_all_redemptions_by_reward_id,
        get_redemption_by_onchain_tx_hash, get_redeption_count,
    },
};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

/// This function is used to serve the graphQL server and GraphiQL IDE.
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

/// This function is used to run the chronicle server.
/// `[DB]` This is a generic type, which is used to store the database.
/// `[Query]` This is a gaint Query entity, for all the Events enitities and all the tx enitities.
pub async fn run_chronicle_server<Query>(
    config: ServerConfig,
    query: Query,
    db_url: String,
) -> Result<(), anyhow::Error>
where
    Query: ObjectType + 'static,
{
    let url = config.server_url.clone();
    let schema = Schema::build(query, EmptyMutation, EmptySubscription)
        .data(db_url.clone())
        .finish();

    let app_state = Arc::new(AppState {
        db_url: db_url.clone(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "Open reward Indexer." }))
        // graphql routes
        .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        // pool routes
        .route("/get-all-pools", get(get_all_pools))
        .route(
            "/get-pool-by-reward-address/:reward_address",
            get(get_pool_by_reward_address),
        )
        .route("/get-pool-count", get(get_pool_count))
        // brand routes
        .route("/get-brand-by-name/:brand_name", get(get_brand_by_name))
        .route("/get-brand-by-id/:brand_id", get(get_brand_by_id))
        .route("/get-brand-count", get(get_brand_count))
        .route("/get-all-brands", get(get_all_brands_paginated))
        // redemption routes
        .route("/get-all-redemption", get(get_all_redemptions))
        .route(
            "/get-all-redemption-by-reward-id/:reward_address",
            get(get_all_redemptions_by_reward_id),
        )
        .route("/get-redeption-count", get(get_redeption_count))
        .route(
            "/get-all-redemption-by-user-address/:user_address",
            get(get_all_redemption_by_user_address),
        )
        .route(
            "/get-redeption-by-onchain-tx-hash/:onchain_tx_hash",
            get(get_redemption_by_onchain_tx_hash),
        )
        // reward routes
        .route("/get-reward-count", get(get_reward_count))
        .route(
            "/get-reward-by-reward-address/:reward_address",
            get(get_reward_by_reward_address),
        )
        .route(
            "/get-reward-by-brand-id/:brand_id",
            get(get_reward_by_brand_id),
        )
        .route("/get-all-rewards", get(get_all_rewards))
        // misc
        .layer(cors)
        .with_state(app_state);

    tracing::info!(url);
    axum::serve(TcpListener::bind(url).await.unwrap(), app)
        .await
        .unwrap();

    Ok(())
}

pub struct AppState {
    pub db_url: String,
}
