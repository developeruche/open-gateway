use alloy::{dyn_abi::DynSolType, hex::ToHexExt};
use anyhow::Ok;

use crate::{
    db::entities::reward::{create_reward_if_does_not_exist, create_reward_table},
    indexer::ChronicleEvent,
    utils::decode_event,
};

pub async fn on_reward_creation(
    event: &ChronicleEvent,
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    // ===============================
    // Operations on the Reward entity
    // ===============================

    create_reward_table(db_client).await?;

    let decoded_event = decode_event(
        event.topics.clone(),
        event.data.clone(),
        DynSolType::Tuple(vec![
            DynSolType::FixedBytes(10),
            DynSolType::Address,
            DynSolType::Address,
            DynSolType::Uint(256),
            DynSolType::Uint(256),
        ]),
        vec![],
    )
    .expect("Failed to decode event: on_reward_created_sucessfully");

    let brand_id = decoded_event.body[0]
        .as_fixed_bytes()
        .expect("Failed to decode event: brand_id")
        .0;

    let reward_address = decoded_event.body[1]
        .as_address()
        .expect("Failed to decode event: ");

    let requestor_address = decoded_event.body[2]
        .as_address()
        .expect("Failed to decode event: requestor address");

    let initial_supply = decoded_event.body[3]
        .as_uint()
        .expect("Failed to decode initial_supply of reward")
        .0;

    let timestamp = decoded_event.body[4]
        .as_uint()
        .expect("Failed to decode event: timestamp")
        .0;

    create_reward_if_does_not_exist(
        brand_id.encode_hex_with_prefix(),
        reward_address.to_string(),
        requestor_address.to_string(),
        initial_supply.to_string(),
        timestamp.to_string(),
        db_client,
    )
    .await?;

    Ok(())
}
