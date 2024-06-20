use backpack::*;

fn main() {
    let stream_name = StreamName::from("depth.SOL_USDC".to_string());
    let mut stream = parse_stream_name(stream_name);
    let mut rng = rand::thread_rng();
    stream.update(5000, &mut rng);
    let message = stream.to_message();
    println!("{:?}", message);
}
