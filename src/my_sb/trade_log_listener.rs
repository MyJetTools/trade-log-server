use std::{collections::BTreeMap, sync::Arc};

use service_sdk::{
    async_trait,
    my_service_bus::abstractions::{
        subscriber::{MessagesReader, MySbSubscriberHandleError, SubscriberCallback},
        MessageId,
    },
    my_telemetry::MyTelemetryContext,
};
use trade_log::contracts::TradeLogSbModel;

use crate::{app::AppContext, psql::TradeLogDbModel};

pub struct TradeLogSbListener {
    pub app: Arc<AppContext>,
}

impl TradeLogSbListener {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }

    async fn save_to_db(
        &self,
        entities: BTreeMap<MessageId, (TradeLogDbModel, MyTelemetryContext)>,
        aggregated_telemetry: Option<&MyTelemetryContext>,
    ) -> Vec<MessageId> {
        let bulk_insert: Vec<_> = entities.values().map(|itm| itm.0.clone()).collect();
        let result = self
            .app
            .repo
            .bulk_insert(bulk_insert.as_slice(), aggregated_telemetry)
            .await;

        if result.is_ok() {
            return vec![];
        }

        let err = result.unwrap_err();
        println!(
            "Could not insert to db as bulk insert. Inserting by one: {:#?}",
            err
        );

        let mut not_inserted = Vec::new();

        for (message_id, (entity, my_telemetry)) in entities {
            let result = self.app.repo.add_log(&entity, Some(&my_telemetry)).await;

            if result.is_err() {
                not_inserted.push(message_id);
            }
        }

        not_inserted
    }
}

#[async_trait::async_trait]
impl SubscriberCallback<TradeLogSbModel> for TradeLogSbListener {
    async fn handle_messages(
        &self,
        messages_reader: &mut MessagesReader<TradeLogSbModel>,
    ) -> Result<(), MySbSubscriberHandleError> {
        let mut entities: BTreeMap<MessageId, (TradeLogDbModel, MyTelemetryContext)> =
            BTreeMap::new();
        let mut aggregated_telemetry: Option<MyTelemetryContext> = None;
        while let Some(message) = messages_reader.get_next_message() {
            let message_id = message.id;
            let telemetry = message.my_telemetry.engage_telemetry();

            match aggregated_telemetry.as_mut() {
                Some(my_telemetry) => my_telemetry.merge_process(&telemetry),
                None => {
                    aggregated_telemetry = Some(telemetry.clone());
                }
            }
            let operation = message.take_message();
            entities.insert(message_id, (operation.into(), telemetry));
        }

        let not_inserted = self
            .save_to_db(entities, aggregated_telemetry.as_ref())
            .await;

        for not_inserted in not_inserted {
            messages_reader.mark_as_not_delivered(not_inserted);
        }

        Ok(())
    }
}
