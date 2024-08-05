use std::sync::Arc;

use crate::{psql::TradeLogRepository, settings::SettingsReader};

pub struct AppContext {
    pub repo: TradeLogRepository,
    pub settings_reader: Arc<SettingsReader>,
}

impl AppContext {
    pub async fn new(settings_reader: Arc<SettingsReader>) -> Self {
        let repo = TradeLogRepository::new(&settings_reader).await;
        Self {
            repo,
            settings_reader,
        }
    }
}
