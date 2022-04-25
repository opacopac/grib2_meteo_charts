#[derive(Debug)]
pub enum MeteoParameterCategory {
    Moisture,
    Cloud,
    Missing,
    Unknown(u8),
}
