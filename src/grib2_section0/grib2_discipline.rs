#[derive(PartialEq, Debug)]
pub enum Grib2Discipline {
    Meteorological,
    Hydrological,
    LandSurface,
    SatelliteRemoteSensing,
    SpaceWeather,
    Oceanographic,
    Missing,
    Unknown(u8)
}
