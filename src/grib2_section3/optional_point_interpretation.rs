#[derive(PartialEq, Debug)]
pub enum OptionalPointInterpretation {
    None,
    FullCoordinateCircles,
    CoordinateLines,
    ActualLatitude,
    Missing,
    Unknown(u8),
}
