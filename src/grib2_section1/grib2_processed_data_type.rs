#[derive(PartialEq, Debug)]
pub enum Grib2ProcessedDataType {
    Analysis,
    Forecast,
    AnalysisAndForecast,
    ControlForecast,
    PerturbedForecast,
    ControlAndPerturbedForecast,
    ProcessedSatelliteObservations,
    ProcessedRadarObservations,
    EventProbability,
    Experimental,
    Missing,
    Unknown(u8),
}
