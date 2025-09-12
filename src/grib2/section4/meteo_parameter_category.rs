#[derive(Debug, PartialEq, Clone)]
pub enum MeteoParameterCategory {
    Temperature,
    Moisture,
    Momentum,
    Mass,
    Cloud,
    Miscellaneous,
    Missing,
    Unknown(u8),
}
