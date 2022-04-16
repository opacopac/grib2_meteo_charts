#[derive(PartialEq, Debug)]
pub enum Grib2RefTimeSignificance {
    Analysis,
    StartOfForecast,
    VerifyingTimeOfForecast,
    ObservationTime,
    Unknown(u8),
    Missing,
}
