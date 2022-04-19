#[derive(PartialEq, Debug)]
pub enum ShapeOfEarth {
    SphericalRadius6371229,
    Missing,
    Unknown(u8),
}
