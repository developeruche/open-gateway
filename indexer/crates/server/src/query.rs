use async_graphql::{Context, Object};
use chronicle_primitives::{
    db::raw_chronicle_event::{
        create_db_instance, get_all_events, get_events_by_block_number, get_events_by_tx_hash,
    },
    indexer::DisplayChronicleEvent,
};

pub struct ChronicleQuery;

#[Object]
impl ChronicleQuery {
    async fn get_all_events<'a>(
        &self,
        cxt: &Context<'a>,
        name: String,
    ) -> Vec<DisplayChronicleEvent> {
        let db_url = cxt.data_unchecked::<String>();
        let mut db_client = create_db_instance(&db_url)
            .await
            .expect("Could not connect to the db");
        let events = get_all_events(&mut db_client, &name)
            .await
            .expect("Could not get events from db");

        events
    }

    async fn get_events_by_tx_hash<'a>(
        &self,
        cxt: &Context<'a>,
        name: String,
        transaction_hash: String,
    ) -> Vec<DisplayChronicleEvent> {
        let db_url = cxt.data_unchecked::<String>();
        let mut db_client = create_db_instance(&db_url)
            .await
            .expect("Could not connect to the db");
        let events = get_events_by_tx_hash(&mut db_client, &name, transaction_hash)
            .await
            .expect("Could not get events from db");

        events
    }

    async fn get_events_by_block_number<'a>(
        &self,
        cxt: &Context<'a>,
        name: String,
        block_number: String,
    ) -> Vec<DisplayChronicleEvent> {
        let db_url = cxt.data_unchecked::<String>();
        let mut db_client = create_db_instance(&db_url)
            .await
            .expect("Could not connect to the db");
        let events = get_events_by_block_number(&mut db_client, &name, block_number)
            .await
            .expect("Could not get events from db");

        events
    }
}
