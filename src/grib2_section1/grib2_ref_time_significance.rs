#[derive(PartialEq, Debug)]
pub enum Grib2RefTimeSignificance {
    Analysis,
    StartOfForecast,
    VerifyingTimeOfForecast,
    ObservationTime,
    Missing,
    Unknown(u8),
}
