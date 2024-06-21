use super::event_type::EventType;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::SolUsd => write!(f, "SOL_USD"),
            Symbol::SolUsdc => write!(f, "SOL_USDC"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl Display for StreamName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.interval {
            Some(interval) => write!(f, "{}.{}.{}", self.stream, interval, self.symbol),
            None => write!(f, "{}.{}", self.stream, self.symbol),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    #[serde(rename = "SUBSCRIBE")]
    Subscribe,
    #[serde(rename = "UNSUBSCRIBE")]
    Unsubscribe,
}

impl From<String> for Method {
    fn from(method: String) -> Self {
        match method.as_str() {
            "SUBSCRIBE" => Method::Subscribe,
            "UNSUBSCRIBE" => Method::Unsubscribe,
            _ => panic!("Invalid method type"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribStream {
    pub method: Method,
    pub params: Vec<StreamName>,
}
