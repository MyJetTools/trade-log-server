use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_postgres!();

#[derive(TableSchema, InsertDbEntity, UpdateDbEntity, SelectDbEntity, Debug, Clone)]
pub struct TradeLogDbModel {
    #[primary_key(0)]
    pub trader_id: String,
    #[primary_key(1)]
    pub account_id: String,
    #[sql_type("timestamp")]
    #[primary_key(2)]
    #[db_index(id:0, index_name:"trade_log_date_idx", is_unique:false, order:"ASC")]
    pub date: DateTimeAsMicroseconds,
    #[primary_key(3)]
    pub component: String,
    pub operation_id: Option<String>,
    pub process_id: Option<String>,
    pub message: String,
    #[json]
    pub data: Vec<TradeLogDbDataModel>,
}

#[derive(WhereDbModel, Debug)]
pub struct QueryTradeLog {
    #[ignore_if_none]
    pub trader_id: Option<String>,
    #[ignore_if_none]
    pub account_id: Option<String>,
    #[ignore_if_none]
    pub component: Option<String>,
    #[ignore_if_none]
    pub process_id: Option<String>,
    #[ignore_if_none]
    pub operation_id: Option<String>,
    #[sql_type("timestamp")]
    #[db_column_name("date")]
    #[operator(">=")]
    #[ignore_if_none]
    pub date_from: Option<DateTimeAsMicroseconds>,
    #[db_column_name("date")]
    #[sql_type("timestamp")]
    #[operator("<")]
    #[ignore_if_none]
    pub date_to: Option<DateTimeAsMicroseconds>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeLogDbDataModel {
    pub key: String,
    pub value: String,
}

#[derive(Debug, WhereDbModel)]
pub struct GcWhereModel {
    #[operator("<")]
    #[sql_type("timestamp")]
    pub date: DateTimeAsMicroseconds,
}
