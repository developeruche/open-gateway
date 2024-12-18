use alloy::{providers::ProviderBuilder, rpc::client::WsConnect};
use anyhow::bail;
use async_trait::async_trait;
use chronicle_indexer::events::evm::EvmEventIndexer;
use chronicle_primitives::{
    db::raw_chronicle_event::create_db_instance, interfaces::ChronicleEventIndexer, IndexerConfig,
    StateMachine,
};
use tokio::{select, try_join};
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::Task;

#[derive(Debug)]
pub struct IndexerTask {
    pub config: IndexerConfig,
    pub db_url: String,
}

#[async_trait]
impl Task for IndexerTask {
    async fn run(mut self: Box<Self>, shutdown_token: CancellationToken) -> anyhow::Result<()> {
        let mut client = create_db_instance(&self.db_url)
            .await
            .expect("Could not create db instance");
        let ws = WsConnect::new(self.config.rpc_url.clone());
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();
        match self.config.state_machine.clone().into() {
            StateMachine::EVM => {
                let evm_event_indexer = EvmEventIndexer {};

                // This queries events that have happened from this block number and stores them in the database
                // It also subscribes to new events and stores them in the database
                let evm_indexer_handle = tokio::spawn(async move {
                    select! {
                        event_n_sub = evm_event_indexer.query_then_subscribe_to_events(
                            provider.clone(),
                            self.config.address.clone().parse().expect("CONFIG address could not be parsed"),
                            self.config.event_signature.clone().parse().expect("CONFIG event signature is missing"),
                            self.config.block_number.into(),
                            &mut client,
                        ) => {
                            // Want this indexing to halt before
                            if event_n_sub.is_err() {
                                info!("Event subscription error, exitting now. ERROR: {:?}", event_n_sub.err().unwrap());
                            }
                        }
                        _ = shutdown_token.cancelled() => {
                            info!("Shutting down chronicle indexer");
                        }
                    }
                });

                match try_join!(evm_indexer_handle) {
                    Ok(_) => {
                        info!("Server task completed");
                    }
                    Err(e) => bail!("Error running server: {:?}", e),
                }
            }
            StateMachine::RUNTIME => {}
        }
        Ok(())
    }
}

impl IndexerTask {
    pub fn new(config: IndexerConfig, db_url: String) -> Self {
        Self { config, db_url }
    }

    /// Converts the task into a boxed trait object.
    pub fn boxed(self) -> Box<dyn Task> {
        Box::new(self)
    }
}
