use crate::{psql::TradeLogDbModel, trade_log_grpc::TradeLogGrpcModel};

impl Into<TradeLogGrpcModel> for TradeLogDbModel {
    fn into(self) -> TradeLogGrpcModel {
        TradeLogGrpcModel {
            trader_id: self.trader_id,
            account_id: self.account_id,
            component: self.component,
            process_id: self.process_id,
            operation_id: self.operation_id,
            data: self.data.into_iter().map(|x| (x.key, x.value)).collect(),
            date: self.date.unix_microseconds as u64,
            message: self.message
        }
    }
}
