#[derive(PartialEq, Debug)]
pub enum Grib2GridDefinitionSource {
    GridDefinitionTemplate,
    PredeterminedGridDefinition,
    None,
    Unknown(u8),
}
