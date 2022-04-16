#[derive(PartialEq, Debug)]
pub enum Grib2GridDefinitionTemplateType {
    LatLon,
    LatLonRotated,
    LatLonStretched,
    LatLonRotatedAndStretched,
    UnstructuredGrid,
    Missing,
    Unknown(u16),
}
