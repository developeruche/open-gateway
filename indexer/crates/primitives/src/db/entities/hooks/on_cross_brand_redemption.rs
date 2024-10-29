use alloy::dyn_abi::DynSolType;

use crate::{
    db::entities::redemption::{create_redemption, create_redemption_table},
    indexer::ChronicleEvent,
    utils::decode_event,
};

/// On cross brand redeption, the following action would be maded
/// 1. redption entity record would be created
pub async fn on_cross_brand_redemption(
    event: &ChronicleEvent,
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    // ====================================
    // Operations for the Reward Entity
    // ====================================
    create_redemption_table(db_client).await?;

    let decoded_event = decode_event(
        event.topics.clone(),
        event.data.clone(),
        DynSolType::Tuple(vec![
            DynSolType::Address,
            DynSolType::Address,
            DynSolType::Uint(256),
            DynSolType::Uint(256),
            DynSolType::Bool,
            DynSolType::Address,
        ]),
        vec![],
    )
    .expect("Failed to decode event: on_cross_brand_redemption");

    let source_token = decoded_event.body[0]
        .as_address()
        .expect("Failed to decode source_token");
    let dest_token = decoded_event.body[1]
        .as_address()
        .expect("Failed to decode dest_token");
    let source_amount = decoded_event.body[2]
        .as_uint()
        .expect("Failed to decode source_amount")
        .0;
    let dest_amount = decoded_event.body[3]
        .as_uint()
        .expect("Failed to decode source_amount")
        .0;
    let user_address = decoded_event.body[5]
        .as_address()
        .expect("Failed to decode user_address");
    let onchain_tx_hash = event.transaction_hash.clone();
    let redeemed_at = event.block_timestamp.clone();

    create_redemption(
        source_token.to_string(),
        dest_token.to_string(),
        source_amount.to_string(),
        dest_amount.to_string(),
        user_address.to_string(),
        onchain_tx_hash.to_string(),
        redeemed_at.to_string(),
        db_client,
    )
    .await?;

    // ====================================
    // Operations for the "?" Entity
    // ====================================

    Ok(())
}
