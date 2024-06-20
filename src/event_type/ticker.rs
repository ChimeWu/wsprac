use super::EventType;
use crate::subscrib_stream::Symbol;
use crate::UpdataStream;
use rand::rngs::ThreadRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerStream {
    /*
      {
    "e": "ticker",          // Event type
    "E": 1694687692980000,  // Event time in microseconds
    "s": "SOL_USD",         // Symbol
    "o": "18.75",           // First price
    "c": "19.24",           // Last price
    "h": "19.80",           // High price
    "l": "18.50",           // Low price
    "v": "32123",           // Base asset volume
    "V": "928190",          // Quote asset volume
    "n": 93828              // Number of trades
      }
      */
    #[serde(rename = "e")]
    event_type: EventType,
    #[serde(rename = "E")]
    event_time: u64,
    #[serde(rename = "s")]
    symbol: Symbol,
    #[serde(rename = "o")]
    first_price: String,
    #[serde(rename = "c")]
    last_price: String,
    #[serde(rename = "h")]
    high_price: String,
    #[serde(rename = "l")]
    low_price: String,
    #[serde(rename = "v")]
    base_asset_volume: String,
    #[serde(rename = "V")]
    quote_asset_volume: String,
    #[serde(rename = "n")]
    number_of_trades: u64,
}

impl TickerStream {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            event_type: EventType::Ticker,
            event_time: 0,
            symbol,
            first_price: "0.0".to_string(),
            last_price: "0.0".to_string(),
            high_price: "0.0".to_string(),
            low_price: "0.0".to_string(),
            base_asset_volume: "0.0".to_string(),
            quote_asset_volume: "0.0".to_string(),
            number_of_trades: 0,
        }
    }
}

impl UpdataStream for TickerStream {
    fn update(&mut self, time: u64, rng: &mut ThreadRng) {
        self.event_time = time;
        self.first_price = rng.gen_range(140.0..190.0).to_string();
        self.last_price = rng.gen_range(140.0..190.0).to_string();
        self.high_price = rng.gen_range(140.0..190.0).to_string();
        self.low_price = rng.gen_range(140.0..190.0).to_string();
        self.base_asset_volume = rng.gen_range(0.0..10.0).to_string();
        self.quote_asset_volume = rng.gen_range(0.0..10.0).to_string();
        self.number_of_trades += 1;
    }

    fn to_message(&self) -> tokio_tungstenite::tungstenite::Message {
        let message = serde_json::to_string_pretty(self).unwrap();
        tokio_tungstenite::tungstenite::Message::Text(message)
    }
}
