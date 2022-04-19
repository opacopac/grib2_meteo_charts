#[derive(PartialEq, Debug)]
pub enum GridDefinitionSource {
    GridDefinitionTemplate,
    PredeterminedGridDefinition,
    None,
    Unknown(u8),
}
