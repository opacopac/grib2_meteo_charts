#[derive(PartialEq, Debug)]
pub enum OriginalFieldType {
    FloatingPoint,
    Integer,
    Missing,
    Unknown(u8),
}
