use rand::{rngs::ThreadRng, Rng};

pub mod event_type;
pub mod subscrib_stream;

pub use event_type::*;
pub use subscrib_stream::*;

pub trait UpdataStream {
    fn update(&mut self, time: u64, rng: &mut ThreadRng);
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
