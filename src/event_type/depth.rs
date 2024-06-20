use crate::*;
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

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
