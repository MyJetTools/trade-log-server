use std::sync::Arc;

use service_sdk::{
    my_postgres::MyPostgres, my_telemetry::MyTelemetryContext,
    rust_extensions::date_time::DateTimeAsMicroseconds, ServiceInfo,
};

use crate::settings::SettingsReader;

use super::{GcWhereModel, QueryTradeLog, TradeLogDbModel};

const TABLE_NAME: &str = "trade_log";
const TABLE_NAME_PK: &str = "trade_log_pk";

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
                Some(TABLE_NAME_PK.into()),
            )
            .build()
            .await,
        }
    }

    pub async fn add_logs(
        &self,
        entities: &[TradeLogDbModel],
        telemetry: Option<&MyTelemetryContext>,
    ) {
        self.postgres
            .bulk_insert_or_update_db_entity(
                TABLE_NAME,
                service_sdk::my_postgres::UpdateConflictType::OnPrimaryKeyConstraint(
                    TABLE_NAME_PK.into(),
                ),
                entities,
                telemetry,
            )
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

    pub async fn gc(&self, from: DateTimeAsMicroseconds) {
        let where_model = GcWhereModel { date: from };
        self.postgres
            .delete_db_entity(TABLE_NAME, &where_model, None)
            .await
            .unwrap()
    }
}
