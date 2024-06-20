use super::event_type::EventType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Symbol {
    #[serde(rename = "SOL_USD")]
    SolUsd,
    #[serde(rename = "SOL_USDC")]
    SolUsdc,
}

impl From<String> for Symbol {
    fn from(symbol: String) -> Self {
        match symbol.as_str() {
            "SOL_USD" => Symbol::SolUsd,
            "SOL_USDC" => Symbol::SolUsdc,
            _ => panic!("Invalid symbol type"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamName {
    pub stream: EventType,
    pub interval: Option<String>,
    pub symbol: Symbol,
}

impl StreamName {
    pub fn new() -> Self {
        Self {
            stream: EventType::BookTicker,
            interval: None,
            symbol: Symbol::SolUsd,
        }
    }
}

impl Default for StreamName {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for StreamName {
    fn from(stream_name: String) -> Self {
        let mut stream_name = stream_name.split('.');
        let len = stream_name.clone().count();
        if len == 2 {
            let stream: EventType = stream_name.next().unwrap().to_string().into();
            let symbol: Symbol = stream_name.next().unwrap().to_string().into();
            StreamName {
                stream,
                interval: None,
                symbol,
            }
        } else {
            let stream: EventType = stream_name.next().unwrap().to_string().into();
            let interval: Option<String> = Some(stream_name.next().unwrap().to_string());
            let symbol: Symbol = stream_name.next().unwrap().to_string().into();
            StreamName {
                stream,
                interval,
                symbol,
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    #[serde(rename = "subscribe")]
    Subscribe,
    #[serde(rename = "unsubscribe")]
    Unsubscribe,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribStream {
    method: Method,
    params: Vec<StreamName>,
}
