use std::sync::Arc;

use service_sdk::{
    async_trait,
    my_service_bus::abstractions::subscriber::{
        MessagesReader, MySbSubscriberHandleError, SubscriberCallback,
    },
};
use trade_log::contracts::TradeLogSbModel;

use crate::psql::TradeLogRepository;

pub struct TradeLogSbListener {
    pub repo: Arc<TradeLogRepository>,
}

impl TradeLogSbListener {
    pub fn new(repo: Arc<TradeLogRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl SubscriberCallback<TradeLogSbModel> for TradeLogSbListener {
    async fn handle_messages(
        &self,
        messages_reader: &mut MessagesReader<TradeLogSbModel>,
    ) -> Result<(), MySbSubscriberHandleError> {
        while let Some(message) = messages_reader.get_next_message() {
            let telemetry = message.my_telemetry.engage_telemetry();
            let operation = message.take_message();
            self.repo.add_log(operation.into(), &telemetry).await;
        }

        Ok(())
    }
}
