use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};

pub trait UpdataStream {
    fn update(&mut self, time: u64, rng: &mut ThreadRng);
}

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
    stream: EventType,
    interval: Option<String>,
    symbol: Symbol,
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
}

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
    fn update(&mut self, time: u64, rng: &mut ThreadRng) {
        self.event_time = time;
        self.engine_timestamp = time - rng.gen_range(0..1000);
        self.price = rng.gen_range(140.0..190.0).to_string();
        self.quantity = rng.gen_range(0.0..10.0).to_string();
        self.buyer_order_id = rng.gen_range(1000000..9999999).to_string();
        self.seller_order_id = rng.gen_range(1000000..9999999).to_string();
        self.trade_id += 1;
        self.is_buyer_the_maker = rng.gen_bool(0.5);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthStream {
    /*
      {
    "e": "depth",           // Event type
    "E": 1694687965941000,  // Event time in microseconds
    "s": "SOL_USDC",        // Symbol
    "a": [                  // Asks
      [
        "18.70",
        "0.000"
      ]
    ],
    "b": [                  // Bids
      [
        "18.67",
        "0.832"
      ],
      [
        "18.68",
        "0.000"
      ]
    ],
    "U": 94978271,          // First update ID in event
    "u": 94978271,          // Final update ID in event
    "T": 1694687965940999   // Engine timestamp in microseconds
      }
       */
    #[serde(rename = "e")]
    event_type: EventType,
    #[serde(rename = "E")]
    event_time: u64,
    #[serde(rename = "s")]
    symbol: Symbol,
    #[serde(rename = "a")]
    asks: Vec<Vec<String>>,
    #[serde(rename = "b")]
    bids: Vec<Vec<String>>,
    #[serde(rename = "U")]
    first_update_id: u64,
    #[serde(rename = "u")]
    final_update_id: u64,
    #[serde(rename = "T")]
    engine_timestamp: u64,
}

impl DepthStream {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            event_type: EventType::Depth,
            event_time: 0,
            symbol,
            asks: vec![vec!["0.0".to_string(), "0.0".to_string()]],
            bids: vec![vec!["0.0".to_string(), "0.0".to_string()]],
            first_update_id: 0,
            final_update_id: 0,
            engine_timestamp: 0,
        }
    }
}

impl UpdataStream for DepthStream {
    fn update(&mut self, time: u64, rng: &mut ThreadRng) {
        self.event_time = time;
        self.engine_timestamp = time - rng.gen_range(0..1000);
        self.first_update_id += 1;
        self.final_update_id += 1;
        self.asks = vec![vec![
            rng.gen_range(140.0..190.0).to_string(),
            rng.gen_range(0.0..10.0).to_string(),
        ]];
        self.bids = vec![vec![
            (self.asks[0][0].parse::<f64>().unwrap() - rng.gen_range(0.0..10.0)).to_string(),
            (self.asks[0][1].parse::<f64>().unwrap() - rng.gen_range(0.0..10.0)).to_string(),
        ]];
    }
}

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
    fn update(&mut self, time: u64, rng: &mut ThreadRng) {
        self.event_time = time;
        self.engine_timestamp = time - rng.gen_range(0..1000);
        self.inside_ask_price = rng.gen_range(140.0..190.0).to_string();
        self.inside_ask_quantity = rng.gen_range(0.0..10.0).to_string();
        self.inside_bid_price = rng.gen_range(140.0..190.0).to_string();
        self.inside_bid_quantity = rng.gen_range(0.0..10.0).to_string();
        self.update_id = rng.gen_range(1000000..9999999).to_string();
    }
}

pub fn parse_stream_name(stream_name: StreamName) -> Box<dyn UpdataStream> {
    match stream_name.stream {
        EventType::Kline => Box::new(KLineStream::new(stream_name.symbol)),
        EventType::Ticker => Box::new(TickerStream::new(stream_name.symbol)),
        EventType::Trade => Box::new(TradeStream::new(stream_name.symbol)),
        EventType::Depth => Box::new(DepthStream::new(stream_name.symbol)),
        EventType::BookTicker => Box::new(BookTickerStream::new(stream_name.symbol)),
    }
}
