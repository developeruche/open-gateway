use async_trait::async_trait;

use crate::indexer::ChronicleTransaction;

/// This event indexer triat would be shared across all supported chains
#[async_trait]
pub trait ChronicleEventIndexer {
    type SubProvider;
    type ContractAddress;
    type EventSignature;
    type BlockNumber;
    type EventDecoder;

    /// This function queries events from a specified block number
    /// `[Filter]`, having `address`, `last_block` and `event_signature` as parameters
    async fn query_then_subscribe_to_events(
        &self,
        provider: Self::SubProvider,
        addr: Self::ContractAddress,
        event_sig: Self::EventSignature,
        block_nuber: Self::BlockNumber,
        db_client: &mut tokio_postgres::Client,
    ) -> Result<(), anyhow::Error>;

    /// This creates a filter and subscribes to an event returning the event
    /// stream <T: Stream<Item = Resp> + Unpin>
    async fn subscribe_to_events(
        &self,
        provider: Self::SubProvider,
        addr: Vec<Self::ContractAddress>,
        event_sig: Self::EventSignature,
        db_client: &mut tokio_postgres::Client,
    ) -> Result<(), anyhow::Error>;
}

/// This transaction indexer trait would be used across all supported chains
#[async_trait]
pub trait ChronicleTransactionIndexer {
    type SubProvider;
    type TargetAddress;

    /// This function subscribes to blocks and filters transactions based on the index address.
    /// Uses a callback closure to output the filter tx
    async fn subscribe_transactions<F>(
        &self,
        index_address: Self::TargetAddress,
        provider: Self::SubProvider,
        callback: F,
    ) -> Result<(), anyhow::Error>
    where
        F: FnMut(Vec<ChronicleTransaction>);
}
