use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};
use service_sdk::my_postgres;
use service_sdk::my_postgres::macros::{InsertDbEntity, SelectDbEntity, TableSchema, WhereDbModel};

#[derive(TableSchema, InsertDbEntity, SelectDbEntity, Debug, Clone)]
pub struct TradeLogDbModel {
    pub trader_id: String,
    pub account_id: String,
    pub component: String,
    pub process_id: Option<String>,
    pub operation_id: Option<String>,
    pub message: String,
    #[json]
    pub data: Vec<TradeLogDbDataModel>,
    #[sql_type("timestamp")]
    pub date: DateTimeAsMicroseconds,
}

#[derive(WhereDbModel)]
pub struct QueryTradeLog {
    pub trader_id: Option<String>,
    pub account_id: Option<String>,
    pub component: Option<String>,
    pub process_id: Option<String>,
    pub operation_id: Option<String>,
    #[sql_type("timestamp")]
    pub date_from: Option<DateTimeAsMicroseconds>,
    #[sql_type("timestamp")]
    pub date_to: Option<DateTimeAsMicroseconds>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeLogDbDataModel {
    pub key: String,
    pub value: String,
}
