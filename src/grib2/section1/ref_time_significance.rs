#[derive(PartialEq, Debug)]
pub enum RefTimeSignificance {
    Analysis,
    StartOfForecast,
    VerifyingTimeOfForecast,
    ObservationTime,
    Missing,
    Unknown(u8),
}
