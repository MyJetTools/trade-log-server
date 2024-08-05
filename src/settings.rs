use serde::{Deserialize, Serialize};
use service_sdk::async_trait;

service_sdk::macros::use_settings!();
#[derive(
    my_settings_reader::SettingsModel,
    AutoGenerateSettingsTraits,
    SdkSettingsTraits,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct SettingsModel {
    pub postgres_conn_string: String,
    pub my_sb_tcp_host_port: String,
    pub my_telemetry: String,
    pub seq_conn_string: String,
    pub gc_days: u32,
}

impl SettingsReader {
    pub async fn get_gc_days(&self) -> u32 {
        let read_access = self.settings.read().await;
        read_access.gc_days
    }
}
