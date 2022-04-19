#[derive(PartialEq, Debug)]
pub enum ProcessedDataType {
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
