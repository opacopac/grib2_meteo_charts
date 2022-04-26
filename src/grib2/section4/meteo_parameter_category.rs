#[derive(Debug, PartialEq, Clone)]
pub enum MeteoParameterCategory {
    Moisture,
    Momentum,
    Cloud,
    Missing,
    Unknown(u8),
}
