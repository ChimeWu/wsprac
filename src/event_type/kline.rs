use crate::*;
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KLineStream {
    /*
      {
    "e": "kline",           // Event type
    "E": 1694687692980000,  // Event time in microseconds
    "s": "SOL_USD",         // Symbol
    "t": 123400000,         // K-Line start time in seconds
    "T": 123460000,         // K-Line close time in seconds
    "o": "18.75",           // Open price
    "c": "19.25",           // Close price
    "h": "19.80",           // High price
    "l": "18.50",           // Low price
    "v": "32123",           // Base asset volume
    "n": 93828,             // Number of trades
    "X": false              // Is this k-line closed?
      }
       */
    #[serde(rename = "e")]
    event_type: EventType,
    #[serde(rename = "E")]
    event_time: u64,
    #[serde(rename = "s")]
    symbol: Symbol,
    #[serde(rename = "t")]
    kline_start_time: u64,
    #[serde(rename = "T")]
    kline_close_time: u64,
    #[serde(rename = "o")]
    open_price: String,
    #[serde(rename = "c")]
    close_price: String,
    #[serde(rename = "h")]
    high_price: String,
    #[serde(rename = "l")]
    low_price: String,
    #[serde(rename = "v")]
    base_asset_volume: String,
    #[serde(rename = "n")]
    number_of_trades: u64,
    #[serde(rename = "X")]
    is_kline_closed: bool,
}

impl KLineStream {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            event_type: EventType::Kline,
            event_time: 0,
            symbol,
            kline_start_time: 0,
            kline_close_time: 0,
            open_price: "0.0".to_string(),
            close_price: "0.0".to_string(),
            high_price: "0.0".to_string(),
            low_price: "0.0".to_string(),
            base_asset_volume: "0.0".to_string(),
            number_of_trades: 0,
            is_kline_closed: false,
        }
    }
}

impl UpdataStream for KLineStream {
    fn update(&mut self, time: u64, rng: &mut ThreadRng) {
        self.event_time = time;
        self.kline_start_time = time - rng.gen_range(0..1000);
        self.kline_close_time = time;
        self.open_price = rng.gen_range(140.0..190.0).to_string();
        self.close_price = rng.gen_range(140.0..190.0).to_string();
        self.high_price = rng.gen_range(140.0..190.0).to_string();
        self.low_price = rng.gen_range(140.0..190.0).to_string();
        self.base_asset_volume = rng.gen_range(0.0..10.0).to_string();
        self.number_of_trades += 1;
        self.is_kline_closed = rng.gen_bool(0.5);
    }
}
