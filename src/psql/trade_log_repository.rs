use std::sync::Arc;

use service_sdk::{my_postgres::MyPostgres, my_telemetry::MyTelemetryContext, ServiceInfo};

use crate::settings::SettingsReader;

use super::{QueryTradeLog, TradeLogDbModel};

const TABLE_NAME: &str = "trade_log";

pub struct TradeLogRepository {
    postgres: MyPostgres,
}

impl TradeLogRepository {
    pub async fn new(settings_reader: &Arc<SettingsReader>) -> Self {
        Self {
            postgres: MyPostgres::from_settings(
                settings_reader.get_service_name(),
                settings_reader.clone(),
                service_sdk::my_logger::LOGGER.clone(),
            )
            .with_table_schema_verification::<TradeLogDbModel>(
                TABLE_NAME,
                Some(format!("tradelog_PK")),
            )
            .build()
            .await,
        }
    }

    pub async fn add_log(&self, log: TradeLogDbModel, telemetry: &MyTelemetryContext) {
        self.postgres
            .insert_db_entity(&log, TABLE_NAME, Some(telemetry))
            .await
            .unwrap();
    }

    pub async fn query(
        &self,
        query: QueryTradeLog,
        telemetry: &MyTelemetryContext,
    ) -> Vec<TradeLogDbModel> {

        println!("query: {:#?}", query);
        self.postgres
            .query_rows(TABLE_NAME, Some(&query), Some(telemetry))
            .await
            .unwrap()
    }
}
