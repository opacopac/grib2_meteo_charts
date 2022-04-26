#[derive(Debug, PartialEq)]
pub enum MeteoParameterCategory {
    Moisture,
    Cloud,
    Missing,
    Unknown(u8),
}
