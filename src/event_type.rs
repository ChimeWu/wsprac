use serde::{Deserialize, Serialize};

pub mod book_ticker;
pub mod depth;
pub mod kline;
pub mod ticker;
pub mod trade;

pub use book_ticker::BookTickerStream;
pub use depth::DepthStream;
pub use kline::KLineStream;
pub use ticker::TickerStream;
pub use trade::TradeStream;

#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
    #[serde(rename = "kline")]
    Kline,
    #[serde(rename = "ticker")]
    Ticker,
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "depth")]
    Depth,
    #[serde(rename = "bookTicker")]
    BookTicker,
}

impl From<String> for EventType {
    fn from(event_type: String) -> Self {
        match event_type.as_str() {
            "kline" => EventType::Kline,
            "ticker" => EventType::Ticker,
            "trade" => EventType::Trade,
            "depth" => EventType::Depth,
            "bookTicker" => EventType::BookTicker,
            _ => panic!("Invalid event type"),
        }
    }
}
