use alloy::{dyn_abi::DynSolType, hex::ToHexExt};
use anyhow::Ok;

use crate::{
    db::entities::brand::{create_brand_if_does_not_exist, create_brand_table},
    indexer::ChronicleEvent,
    utils::decode_event,
};

pub async fn on_register_brand(
    event: &ChronicleEvent,
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    // ============================
    // Operations on the brand entity
    // ============================
    create_brand_table(db_client).await?;

    let decoded_event = decode_event(
        event.topics.clone(),
        event.data.clone(),
        DynSolType::Tuple(vec![
            DynSolType::String,
            DynSolType::String,
            DynSolType::Address,
            DynSolType::FixedBytes(10),
            DynSolType::Address,
        ]),
        vec![],
    )
    .expect("Failed to decode event: on_register_brand");

    let brand_name = decoded_event.body[0]
        .as_str()
        .expect("Failed to decode brand_name");

    let brand_online_presence = decoded_event.body[1]
        .as_str()
        .expect("Failed to decode brand_online_presence");

    let brand_account = decoded_event.body[2]
        .as_address()
        .expect("Failed to decode brand_account");

    let brand_protocol_id = decoded_event.body[3]
        .as_fixed_bytes()
        .expect("Failed to decode brand_protocol_id")
        .0
        .to_vec();

    let original_requestor = decoded_event.body[4]
        .as_address()
        .expect("Failed to decode original_requestor");

    // Checks for the brand entity if it has been added to the database before
    //If it has, then skip
    create_brand_if_does_not_exist(
        brand_name.to_string(),
        brand_online_presence.to_string(),
        brand_account.to_string(),
        brand_protocol_id.encode_hex_with_prefix(),
        original_requestor.to_string(),
        db_client,
    )
    .await?;
    Ok(())
}
