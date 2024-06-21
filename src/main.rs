use backpack::subscrib_stream::*;

fn main() {
    let stream_name = "depth.SOL_USDC".to_string().into();
    let method = Method::Subscribe;
    let subscrib_stream = SubscribStream {
        method,
        params: vec![stream_name],
    };
    let json = serde_json::to_string(&subscrib_stream).unwrap();
    println!("{}", json);
}
