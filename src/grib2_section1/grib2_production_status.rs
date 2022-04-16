#[derive(PartialEq, Debug)]
pub enum Grib2ProductionStatus {
    Operational,
    Test,
    Research,
    ReAnalysis,
    Thorpex,
    ThorpexTest,
    S2sOperational,
    S2sTest,
    Uerra,
    UerraTest,
    Missing,
    Unknown(u8),
}
