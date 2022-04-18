#[derive(PartialEq, Debug)]
pub enum Grib2ShapeOfEarth {
    SphericalRadius6371229,
    Missing,
    Unknown(u8),
}
