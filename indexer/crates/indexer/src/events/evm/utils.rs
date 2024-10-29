use alloy::{
    dyn_abi::{DecodedEvent, DynSolEvent, DynSolType},
    primitives::{Address, Bytes, LogData, B256},
    providers::{Provider, RootProvider},
    pubsub::PubSubFrontend,
    rpc::types::eth::{BlockNumberOrTag, Filter},
};
use chronicle_primitives::{db::entities::pre_entity_store, indexer::ChronicleEvent};
use futures_util::stream::StreamExt;

pub async fn query_events(
    provider: RootProvider<PubSubFrontend>,
    addr: Address,
    event_sig: B256,
    block_number: BlockNumberOrTag,
) -> Result<Vec<ChronicleEvent>, anyhow::Error> {
    let filter = Filter::new()
        .address(addr)
        .event_signature(event_sig)
        .from_block(block_number);
    let log = provider.get_logs(&filter).await?;
    let chronicle_logs: Vec<ChronicleEvent> = log.into_iter().map(|log| log.into()).collect();

    Ok(chronicle_logs)
}

pub async fn subscribe_to_events(
    provider: RootProvider<PubSubFrontend>,
    addr: Vec<Address>,
    event_sig: B256,
    client: &mut tokio_postgres::Client,
) {
    let filter = Filter::new()
        .address(addr)
        .event_signature(event_sig)
        .from_block(BlockNumberOrTag::Latest);

    let sub = provider
        .subscribe_logs(&filter)
        .await
        .expect("Failed to subscribe to logs");
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        pre_entity_store(&log.into(), client, event_sig)
            .await
            .expect("Failed to store event to db");
    }
}

/// This function is used to decode an event
/// params:
/// topics: Vec<B256> - The topics of the event
/// data: Bytes - The data of the event
/// decoder_format: DynSolType - The format of the event; example -> DynSolType::Tuple(
///     vec![
///         DynSolType::Uint(256)
///     ]
///  ),
/// indexed: Vec<DynSolType> - The indexed values of the event; example -> vec![DynSolType::Address]
pub fn decode_event(
    topics: Vec<B256>,
    data: Bytes,
    decoder_format: DynSolType,
    indexed: Vec<DynSolType>,
) -> Result<DecodedEvent, anyhow::Error> {
    let event: DynSolEvent = DynSolEvent::new_unchecked(Some(topics[0]), indexed, decoder_format);
    let log_data = LogData::new_unchecked(topics, data);
    let decoded_event = event.decode_log_data(&log_data, true)?;

    Ok(decoded_event)
}

#[cfg(test)]
pub mod tests {
    use alloy::{
        primitives::{address, b256},
        providers::ProviderBuilder,
        rpc::client::WsConnect,
    };
    use chronicle_primitives::db::raw_chronicle_event::create_db_instance;

    use super::*;

    const DB_URL: &str = "host=localhost user=postgres";

    #[tokio::test]
    #[ignore]
    pub async fn test_query_events_works() {
        let rpc_url = "wss://eth.merkle.io";

        // Create the provider.
        let ws = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

        let block_num = 19664198u64;
        let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
        let tranfer_event_signature =
            b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

        let events = query_events(
            provider,
            uniswap_token_address,
            tranfer_event_signature,
            block_num.into(),
        )
        .await
        .unwrap();

        for log in events {
            println!("Uniswap token logs: {log:?}");
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_raw_subscribe_logs() {
        let rpc_url = "wss://eth.merkle.io";

        // Create the provider.
        let ws = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

        let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
        let transfer_event_signature =
            b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

        let addresses = vec![uniswap_token_address];
        let filter = Filter::new()
            .address(addresses)
            .event_signature(transfer_event_signature)
            .from_block(BlockNumberOrTag::Latest);

        let sub = provider.subscribe_logs(&filter).await.unwrap();
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            println!("Uniswap token logs: {log:?}");
            break;
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_subscribe_events_works() {
        let rpc_url = "wss://eth.merkle.io";

        // Create the provider.
        let ws = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

        let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
        let transfer_event_signature =
            b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

        let mut client = create_db_instance(&DB_URL.into())
            .await
            .expect("Could not create db instance");

        subscribe_to_events(
            provider,
            vec![uniswap_token_address],
            transfer_event_signature,
            &mut client,
        )
        .await;
    }

    #[tokio::test]
    // #[ignore]
    async fn test_raw_subscribe_logs_can_be_decoded() {
        let rpc_url = "wss://eth.merkle.io";

        // Create the provider.
        let ws = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

        let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
        let transfer_event_signature =
            b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

        let addresses = vec![uniswap_token_address];
        let filter = Filter::new()
            .address(addresses)
            .event_signature(transfer_event_signature)
            .from_block(BlockNumberOrTag::Latest);

        let sub = provider.subscribe_logs(&filter).await.unwrap();
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            println!("Uniswap token logs: {log:?}");
            let decoded_log = decode_event(
                log.topics().to_owned(),
                log.inner.data.data,
                DynSolType::Tuple(vec![
                    DynSolType::Address,
                    DynSolType::Address,
                    DynSolType::Uint(256),
                ]),
                vec![DynSolType::Address, DynSolType::Address],
            )
            .unwrap();
            println!("Uniswap token logs: {decoded_log:?}");
            break;
        }
    }
}

// Uniswap token logs: Log { inner: Log { address: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, data: LogData { topics: [0xe1fffcc4923d04b559f4d29a8bfc6cda04eb5b0d3c460751c2402c5c5cc9109c, 0x0000000000000000000000005ddf30555ee9545c8982626b7e3b6f70e5c2635f], data: 0x000000000000000000000000000000000000000000000000015fb7f9b8c38000 } }, block_hash: Some(0xc818d011c0833c20ab2c69c97b7a74f7161ca1ca94631bca72b6db53bc4b04d6), block_number: Some(20819969), block_timestamp: None, transaction_hash: Some(0x811ba4f47d01fd272e3d2802db97f87077b49f5831e0355e6a3dd8844244e0c5), transaction_index: Some(0), log_index: Some(0), removed: false }
