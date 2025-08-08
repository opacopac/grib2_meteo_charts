#[derive(Debug, PartialEq, Clone)]
pub enum MeteoParameterCategory {
    Moisture,
    Momentum,
    Mass,
    Cloud,
    Miscellaneous,
    Missing,
    Unknown(u8),
}
