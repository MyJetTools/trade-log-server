use std::sync::Arc;

use background::TradeLogSbListener;
use psql::TradeLogRepository;
use settings::SettingsReader;

mod background;
mod mappers;
mod psql;
mod settings;

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".mjt").await;
    let settings_reader = Arc::new(settings_reader);

    let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;

    service_context.register_sb_subscribe(Arc::new(TradeLogSbListener::new(
        Arc::new(TradeLogRepository::new(&settings_reader).await),
    )), service_sdk::my_service_bus::abstractions::subscriber::TopicQueueType::PermanentWithSingleConnection).await;

    service_context.start_application().await;
}
