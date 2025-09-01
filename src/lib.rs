mod exchange_outpost;
use crate::exchange_outpost::{FinData, schedule_email};
use extism_pdk::{FnResult, Json, ToBytes, encoding, plugin_fn};
use serde::Serialize;
use ta::{Next, indicators::RelativeStrengthIndex};

#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {}

#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<Output> {
    let ticker = fin_data.get_ticker("symbol_data")?;
    let period = fin_data.get_call_argument::<usize>("period").unwrap_or(14);
    let limit_low = fin_data
        .get_call_argument::<f64>("limit_low")
        .unwrap_or(30.0);
    let limit_high = fin_data
        .get_call_argument::<f64>("limit_high")
        .unwrap_or(70.0);
    let email = fin_data.get_call_argument::<String>("email")?;

    let mut rsi = RelativeStrengthIndex::new(period).unwrap();
    let mut last = 0.0;
    for candle in ticker.candles.iter() {
        last = rsi.next(candle.close);
    }

    if last < limit_low {
        schedule_email(
            &email,
            format!(
                "RSI is below {}: {} for symbol {}",
                limit_low, last, ticker.symbol
            )
            .as_str(),
        )?;
    } else if last > limit_high {
        schedule_email(
            &email,
            format!(
                "RSI is above {}: {} for symbol {}",
                limit_high, last, ticker.symbol
            )
            .as_str(),
        )?;
    }

    Ok(Output {})
}
