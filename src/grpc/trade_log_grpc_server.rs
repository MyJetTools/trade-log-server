use std::{pin::Pin, sync::Arc};

use service_sdk::{futures_core::Stream, my_grpc_extensions::server::with_telemetry};

use crate::{
    app::AppContext,
    mappers::sanitize_csharp_grpc_string,
    psql::QueryTradeLog,
    trade_log_grpc::{
        trade_log_grpc_service_server::TradeLogGrpcService, QueryTradeLogGrpcRequest,
        TradeLogGrpcModel,
    },
};
use service_sdk::my_grpc_extensions;

#[derive(Clone)]
pub struct GrpcService {
    pub app: Arc<AppContext>,
}

impl GrpcService {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[tonic::async_trait]
impl TradeLogGrpcService for GrpcService {
    type QueryStream = Pin<
        Box<dyn Stream<Item = Result<TradeLogGrpcModel, tonic::Status>> + Send + Sync + 'static>,
    >;

    #[with_telemetry]
    async fn query(
        &self,
        request: tonic::Request<QueryTradeLogGrpcRequest>,
    ) -> Result<tonic::Response<Self::QueryStream>, tonic::Status> {
        let request = request.into_inner();
        println!("request: {:#?}", request);
        let date_from = match request.date_from {
            Some(src) => Some(src.into()),
            None => None,
        };

        let date_to = match request.date_to {
            Some(src) => Some(src.into()),
            None => None,
        };

        let query = QueryTradeLog {
            trader_id: sanitize_csharp_grpc_string(request.trader_id),
            account_id: sanitize_csharp_grpc_string(request.account_id),
            component: sanitize_csharp_grpc_string(request.component),
            process_id: sanitize_csharp_grpc_string(request.process_id),
            operation_id: sanitize_csharp_grpc_string(request.operation_id),
            date_from,
            date_to,
        };

        let models = self.app.repo.query(query, &my_telemetry).await;
        let response =
            my_grpc_extensions::grpc_server::send_vec_to_stream(models.into_iter(), |x| x.into())
                .await;
        return response;
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        return Ok(tonic::Response::new(()));
    }
}
