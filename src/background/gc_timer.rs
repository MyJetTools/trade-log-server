use std::{sync::Arc, time::Duration};

use service_sdk::rust_extensions::{date_time::DateTimeAsMicroseconds, MyTimerTick};

use crate::app::AppContext;

pub struct TradeLogGcTimer {
    pub app: Arc<AppContext>,
}

impl TradeLogGcTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for TradeLogGcTimer {
    async fn tick(&self) {
        let days_before_gc = self.app.settings_reader.get_gc_days().await;

        let days_before_gc = Duration::from_secs(60 * 60 * 24 * days_before_gc as u64);

        let date_from = DateTimeAsMicroseconds::now().sub(days_before_gc);

        self.app.repo.gc(date_from).await;
    }
}
