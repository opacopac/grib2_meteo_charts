pub struct TStamp {}

impl TStamp {
    pub fn print_us(text: &str) -> () {
        let duration = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
        let seconds = duration.as_micros() as f64 / 1_000_000.0;
        println!("{:.6} - {}", seconds, text);
    }
}
