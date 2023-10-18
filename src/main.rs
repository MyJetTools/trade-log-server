use std::sync::Arc;

use background::TradeLogSbListener;
use grpc::GrpcService;
use psql::TradeLogRepository;
use settings::SettingsReader;
use trade_log_grpc::trade_log_grpc_service_server::TradeLogGrpcServiceServer;

mod background;
mod grpc;
mod mappers;
mod psql;
mod settings;

pub mod trade_log_grpc {
    tonic::include_proto!("trade_log");
}

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".mjt").await;
    let settings_reader = Arc::new(settings_reader);

    let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;
    let repo = Arc::new(TradeLogRepository::new(&settings_reader).await);
    service_context.register_sb_subscribe(Arc::new(TradeLogSbListener::new(
        Arc::new(TradeLogRepository::new(&settings_reader).await),
    )), service_sdk::my_service_bus::abstractions::subscriber::TopicQueueType::PermanentWithSingleConnection).await;

    service_context.configure_grpc_server(|builder| {
        builder.add_grpc_service(TradeLogGrpcServiceServer::new(GrpcService::new(
            repo.clone(),
        )));
    });

    service_context.start_application().await;
}
