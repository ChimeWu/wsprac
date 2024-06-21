use super::EventType;
use crate::subscrib_stream::Symbol;
use crate::UpdataStream;
use rand::rngs::ThreadRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BookTickerStream {
    /*
      {
    "e": "bookTicker",          // Event type
    "E": 1694687965941000,      // Event time in microseconds
    "s": "SOL_USDC",            // Symbol
    "a": "18.70",               // Inside ask price
    "A": "1.000",               // Inside ask quantity
    "b": "18.67",               // Inside bid price
    "B": "2.000",               // Inside bid quantity
    "u": "111063070525358080",  // Update ID of event
    "T": 1694687965940999       // Engine timestamp in microseconds
      }
       */
    #[serde(rename = "e")]
    event_type: EventType,
    #[serde(rename = "E")]
    event_time: u64,
    #[serde(rename = "s")]
    symbol: Symbol,
    #[serde(rename = "a")]
    inside_ask_price: String,
    #[serde(rename = "A")]
    inside_ask_quantity: String,
    #[serde(rename = "b")]
    inside_bid_price: String,
    #[serde(rename = "B")]
    inside_bid_quantity: String,
    #[serde(rename = "u")]
    update_id: String,
    #[serde(rename = "T")]
    engine_timestamp: u64,
}

impl BookTickerStream {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            event_type: EventType::BookTicker,
            event_time: 0,
            symbol,
            inside_ask_price: "0.0".to_string(),
            inside_ask_quantity: "0.0".to_string(),
            inside_bid_price: "0.0".to_string(),
            inside_bid_quantity: "0.0".to_string(),
            update_id: "0".to_string(),
            engine_timestamp: 0,
        }
    }
}

impl UpdataStream for BookTickerStream {
    fn update(&mut self, time: u64, mut rng: ThreadRng) {
        self.event_time = time;
        self.engine_timestamp = time;
        self.inside_ask_price = rng.gen_range(140.0..190.0).to_string();
        self.inside_ask_quantity = rng.gen_range(0.0..10.0).to_string();
        self.inside_bid_price = rng.gen_range(140.0..190.0).to_string();
        self.inside_bid_quantity = rng.gen_range(0.0..10.0).to_string();
        self.update_id = rng.gen_range(1000000..9999999).to_string();
    }

    fn to_message(&self) -> tokio_tungstenite::tungstenite::Message {
        let message = serde_json::to_string_pretty(self).unwrap();
        tokio_tungstenite::tungstenite::Message::Text(message)
    }
}
