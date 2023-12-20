use trade_log::contracts::{TradeLogSbModel, TradeLogSbModelDataItem};

use crate::psql::{TradeLogDbDataModel, TradeLogDbModel};

impl Into<TradeLogDbModel> for TradeLogSbModel {
    fn into(self) -> TradeLogDbModel {
        TradeLogDbModel {
            trader_id: self.trader_id,
            account_id: self.account_id,
            component: self.component,
            process_id: if self.process_id.is_empty() {
                None
            } else {
                Some(self.process_id)
            },
            operation_id: if self.operation_id.is_empty() {
                None
            } else {
                Some(self.operation_id)
            },
            message: self.message,
            data: self.data.into_iter().map(|x| x.into()).collect(),
            date: self.date_time_unix_micros.into(),
        }
    }
}

impl Into<TradeLogDbDataModel> for TradeLogSbModelDataItem {
    fn into(self) -> TradeLogDbDataModel {
        TradeLogDbDataModel {
            key: self.key,
            value: self.value,
        }
    }
}

pub fn sanitize_csharp_grpc_string(src: Option<String>) -> Option<String> {
    let Some(src) = src else {
        return None;
    };

    let mut str = src;
    str = str.replace("\n", "");
    str = str.replace("$", "");
    str = str.replace("\t", "");

    return Some(str);
}
