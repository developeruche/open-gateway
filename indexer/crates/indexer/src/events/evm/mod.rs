pub mod utils;

use alloy::{
    primitives::{Address, B256},
    providers::RootProvider,
    pubsub::PubSubFrontend,
    rpc::types::eth::BlockNumberOrTag,
};
use async_trait::async_trait;
use chronicle_primitives::{
    db::entities::{
        pre_entity_store,
        system::{create_system_if_does_not_exist, get_last_block_number},
    },
    interfaces::ChronicleEventIndexer,
};

use self::utils::{query_events, subscribe_to_events};

pub struct EvmEventIndexer;

#[async_trait]
impl ChronicleEventIndexer for EvmEventIndexer {
    type SubProvider = RootProvider<PubSubFrontend>;
    type ContractAddress = Address;
    type EventSignature = B256;
    type BlockNumber = BlockNumberOrTag;
    type EventDecoder = ();

    // TODO: This should be renamed to query then subscribe events
    async fn query_then_subscribe_to_events(
        &self,
        provider: Self::SubProvider,
        addr: Self::ContractAddress,
        event_sig: Self::EventSignature,
        block_number: Self::BlockNumber,
        db_client: &mut tokio_postgres::Client,
    ) -> Result<(), anyhow::Error> {
        // TODO: This should check if the system record has bee created,
        // if not create it and store the last indexed block as block_number
        let last_indexed_block: u64 = (get_last_block_number(db_client).await?).parse()?;
        let block_number = if last_indexed_block > block_number.as_number().expect("Infallible") {
            BlockNumberOrTag::Number(last_indexed_block + 1)
        } else {
            create_system_if_does_not_exist(
                block_number.as_number().unwrap_or(0).to_string(),
                db_client,
            )
            .await?;
            block_number
        };

        // Query existing events from the specified block number
        let events = query_events(provider.clone(), addr, event_sig, block_number).await?;

        // Store all this event is the database
        for event in events {
            pre_entity_store(&event, db_client, event_sig).await?;
        }

        // Now subsbribing the events
        self.subscribe_to_events(provider, vec![addr], event_sig, db_client)
            .await?;

        Ok(())
    }

    async fn subscribe_to_events(
        &self,
        provider: Self::SubProvider,
        addr: Vec<Self::ContractAddress>,
        event_sig: Self::EventSignature,
        db_client: &mut tokio_postgres::Client,
    ) -> Result<(), anyhow::Error> {
        subscribe_to_events(provider, addr, event_sig, db_client).await;

        Ok(())
    }
}
