use alloy::primitives::B256;
use hooks::{
    on_cross_brand_redemption::on_cross_brand_redemption,
    on_current_pool_state::on_current_pool_state, on_register_brand::on_register_brand,
    on_reward_created_successfullly::on_reward_creation,
};
use system::update_last_block_number;

use crate::{
    config::{
        EVENT_FOUR_SIGNATURE, EVENT_ONE_SIGNATURE, EVENT_THREE_SIGNATURE, EVENT_TWO_SIGNATURE,
    },
    indexer::ChronicleEvent,
};

pub mod brand;
pub mod hooks;
pub mod pool;
pub mod redemption;
pub mod reward;
pub mod system;

#[cfg(feature = "development")]
pub const DB_VERSION: &str = "_DEVELOPMENT_30";

// Default to production if no feature is specified
#[cfg(not(feature = "development"))]
pub const DB_VERSION: &str = "_PRODUCTION_5";

/// This function is hit anytime an event is recieved by the indexer.
/// using the event hash, the enitity that needs to migrated would be triggered
pub async fn pre_entity_store(
    event: &ChronicleEvent,
    db_client: &mut tokio_postgres::Client,
    event_sig: B256,
) -> Result<(), anyhow::Error> {
    match event_sig {
        EVENT_ONE_SIGNATURE => {
            on_cross_brand_redemption(event, db_client).await?;
        }
        EVENT_TWO_SIGNATURE => {
            on_current_pool_state(event, db_client).await?;
        }
        EVENT_THREE_SIGNATURE => {
            on_register_brand(event, db_client).await?;
        }
        EVENT_FOUR_SIGNATURE => {
            on_reward_creation(event, db_client).await?;
        }
        _ => {}
    }

    update_last_block_number(event.block_number.to_string(), db_client).await?;

    Ok(())
}
