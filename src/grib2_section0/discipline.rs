#[derive(PartialEq, Debug)]
pub enum Discipline {
    Meteorological,
    Hydrological,
    LandSurface,
    SatelliteRemoteSensing,
    SpaceWeather,
    Oceanographic,
    Missing,
    Unknown(u8)
}
