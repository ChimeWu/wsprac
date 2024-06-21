use super::EventType;
use crate::subscrib_stream::Symbol;
use crate::UpdataStream;
use rand::rngs::ThreadRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeStream {
    /*
      {
    "e": "trade",                   // Event type
    "E": 1694688638091000,          // Event time in microseconds
    "s": "SOL_USDC",                // Symbol
    "p": "18.68",                   // Price
    "q": "0.122",                   // Quantity
    "b": "111063114377265150",      // Buyer order ID
    "a": "111063114585735170",      // Seller order ID
    "t": 12345,                     // Trade ID
    "T": 1694688638089000,          // Engine timestamp in microseconds
    "m": true                       // Is the buyer the maker?
      }
       */
    #[serde(rename = "e")]
    event_type: EventType,
    #[serde(rename = "E")]
    event_time: u64,
    #[serde(rename = "s")]
    symbol: Symbol,
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
    #[serde(rename = "b")]
    buyer_order_id: String,
    #[serde(rename = "a")]
    seller_order_id: String,
    #[serde(rename = "t")]
    trade_id: u64,
    #[serde(rename = "T")]
    engine_timestamp: u64,
    #[serde(rename = "m")]
    is_buyer_the_maker: bool,
}

impl TradeStream {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            event_type: EventType::Trade,
            event_time: 0,
            symbol,
            price: "0.0".to_string(),
            quantity: "0.0".to_string(),
            buyer_order_id: "0".to_string(),
            seller_order_id: "0".to_string(),
            trade_id: 0,
            engine_timestamp: 0,
            is_buyer_the_maker: false,
        }
    }
}

impl UpdataStream for TradeStream {
    fn update(&mut self, time: u64, mut rng: ThreadRng) {
        self.event_time = time;
        self.engine_timestamp = time;
        self.price = rng.gen_range(140.0..190.0).to_string();
        self.quantity = rng.gen_range(0.0..10.0).to_string();
        self.buyer_order_id = rng.gen_range(1000000..9999999).to_string();
        self.seller_order_id = rng.gen_range(1000000..9999999).to_string();
        self.trade_id += 1;
        self.is_buyer_the_maker = rng.gen_bool(0.5);
    }

    fn to_message(&self) -> tokio_tungstenite::tungstenite::Message {
        let message = serde_json::to_string_pretty(self).unwrap();
        tokio_tungstenite::tungstenite::Message::Text(message)
    }
}
