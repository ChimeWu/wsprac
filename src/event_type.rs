use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EventType::Kline => write!(f, "kline"),
            EventType::Ticker => write!(f, "ticker"),
            EventType::Trade => write!(f, "trade"),
            EventType::Depth => write!(f, "depth"),
            EventType::BookTicker => write!(f, "bookTicker"),
        }
    }
}
