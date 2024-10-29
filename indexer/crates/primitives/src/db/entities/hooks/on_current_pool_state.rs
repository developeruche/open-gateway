use alloy::{dyn_abi::DynSolType, primitives::Address};

use crate::{
    db::entities::pool::{
        check_pool_exist_by_reward_address, create_pool_if_does_not_exist, create_pool_table,
        update_pool_reward_and_me_amount,
    },
    indexer::ChronicleEvent,
    utils::decode_event,
};

pub async fn on_current_pool_state(
    event: &ChronicleEvent,
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    // ================================
    // Operations on the pool entity
    // ================================
    create_pool_table(db_client).await?;
    // event-signature [`currentPoolsState(address,uint256,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256)`]
    let decoded_event = decode_event(
        event.topics.clone(),
        event.data.clone(),
        DynSolType::Tuple(vec![
            DynSolType::Uint(256),
            DynSolType::Uint(256),
            DynSolType::Uint(256),
            DynSolType::Uint(256),
            DynSolType::Uint(256),
            DynSolType::Uint(256),
            DynSolType::Uint(256),
            DynSolType::Uint(256),
            DynSolType::Uint(256),
            DynSolType::Uint(256),
        ]),
        vec![DynSolType::Address, DynSolType::Address],
    )
    .expect("Failed to decode event: on_cross_brand_redemption");

    let current_amount_of_me_token_reward_one = decoded_event.body[0]
        .as_uint()
        .expect("Failed to decode current_amount_of_me_token_reward_one")
        .0;

    let current_amount_of_reward_token_reward_one = decoded_event.body[1]
        .as_uint()
        .expect("Failed to decode current_amount_of_me_token_reward_one")
        .0;
    let r_optimal_reward_one = decoded_event.body[4]
        .as_uint()
        .expect("Failed to decode r_optimal_reward_one")
        .0;
    let current_amount_of_me_token_reward_two = decoded_event.body[5]
        .as_uint()
        .expect("Failed to decode current_amount_of_me_token_reward_two")
        .0;
    let current_amount_of_reward_token_reward_two = decoded_event.body[6]
        .as_uint()
        .expect("Failed to decode current_amount_of_reward_token_reward_two")
        .0;
    let r_optimal_reward_two = decoded_event.body[7]
        .as_uint()
        .expect("Failed to decode r_optimal_reward_two")
        .0;

    let reward_one_address = decoded_event.indexed[0]
        .as_address()
        .expect("Failed to decode the reward address of pool");

    let reward_two_address = decoded_event.indexed[1]
        .as_address()
        .expect("Failed to decode the reward address of pool");

    // Operating on pool-one
    if check_pool_exist_by_reward_address(reward_one_address.to_string(), db_client).await? {
        // just make update
        update_pool_reward_and_me_amount(
            reward_one_address.to_string(),
            current_amount_of_me_token_reward_one.to_string(),
            current_amount_of_reward_token_reward_one.to_string(),
            db_client,
        )
        .await?;
    } else {
        // create new pool with default data
        create_pool_if_does_not_exist(
            Address::ZERO.to_string(),
            reward_one_address.to_string(),
            Address::ZERO.to_string(),
            current_amount_of_reward_token_reward_one.to_string(),
            current_amount_of_me_token_reward_one.to_string(),
            r_optimal_reward_one.to_string(),
            db_client,
        )
        .await?;
    }

    // Operating on pool-two
    if check_pool_exist_by_reward_address(reward_two_address.to_string(), db_client).await? {
        // just make update
        update_pool_reward_and_me_amount(
            reward_two_address.to_string(),
            current_amount_of_me_token_reward_two.to_string(),
            current_amount_of_reward_token_reward_two.to_string(),
            db_client,
        )
        .await?;
    } else {
        // create new pool with default data
        create_pool_if_does_not_exist(
            Address::ZERO.to_string(),
            reward_two_address.to_string(),
            Address::ZERO.to_string(),
            current_amount_of_reward_token_reward_two.to_string(),
            current_amount_of_me_token_reward_two.to_string(),
            r_optimal_reward_two.to_string(),
            db_client,
        )
        .await?;
    }

    // ====================================
    // Operations for the "?" Entity
    // ====================================

    Ok(())
}
