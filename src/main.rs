use std::{sync::Arc, time::Duration};

use grpc::GrpcService;
use my_sb::TradeLogSbListener;
use settings::SettingsReader;
use trade_log_grpc::trade_log_grpc_service_server::TradeLogGrpcServiceServer;

mod app;
mod background;
mod grpc;
mod mappers;
mod my_sb;
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

    let app = Arc::new(app::AppContext::new(settings_reader).await);

    service_context.register_sb_subscribe(Arc::new(TradeLogSbListener::new(
        app.clone(),
    )), service_sdk::my_service_bus::abstractions::subscriber::TopicQueueType::PermanentWithSingleConnection).await;

    service_context.configure_grpc_server(|builder| {
        builder.add_grpc_service(TradeLogGrpcServiceServer::new(GrpcService::new(
            app.clone(),
        )));
    });

    service_context.register_timer(Duration::from_secs(30), |builder| {
        builder.register_timer(
            "GcTimer",
            Arc::new(background::TradeLogGcTimer::new(app.clone())),
        );
    });

    service_context.start_application().await;
}
