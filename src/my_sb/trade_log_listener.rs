use std::sync::Arc;

use service_sdk::{
    async_trait,
    my_service_bus::abstractions::subscriber::{
        MessagesReader, MySbSubscriberHandleError, SubscriberCallback,
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
}

#[async_trait::async_trait]
impl SubscriberCallback<TradeLogSbModel> for TradeLogSbListener {
    async fn handle_messages(
        &self,
        messages_reader: &mut MessagesReader<TradeLogSbModel>,
    ) -> Result<(), MySbSubscriberHandleError> {
        let mut entities: Vec<TradeLogDbModel> = Vec::new();
        let mut my_telemetry: Option<MyTelemetryContext> = None;
        while let Some(message) = messages_reader.get_next_message() {
            let telemetry = message.my_telemetry.engage_telemetry();

            match my_telemetry.as_mut() {
                Some(my_telemetry) => my_telemetry.merge_process(&telemetry),
                None => {
                    my_telemetry = Some(telemetry);
                }
            }
            let operation = message.take_message();
            entities.push(operation.into());
        }

        self.app
            .repo
            .add_logs(entities.as_slice(), my_telemetry.as_ref())
            .await;

        Ok(())
    }
}
