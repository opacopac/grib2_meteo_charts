#[derive(Debug, PartialEq, Clone)]
pub enum MeteoParameterCategory {
    Moisture,
    Momentum,
    Mass,
    Cloud,
    Missing,
    Unknown(u8),
}
