#[derive(PartialEq, Debug)]
pub enum Grib2OptionalPointInterpretation {
    None,
    FullCoordinateCircles,
    CoordinateLines,
    ActualLatitude,
    Missing,
    Unknown(u8),
}
