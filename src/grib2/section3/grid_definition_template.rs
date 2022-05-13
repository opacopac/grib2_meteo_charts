use crate::grib2::section3::grid_definition_template_3_0::GridDefinitionTemplate3_0;
use crate::grib2::section3::grid_definition_template_3_101::GridDefinitionTemplate3_101;

#[derive(Debug)]
pub enum GridDefinitionTemplate {
    LatitudeLongitude(GridDefinitionTemplate3_0),
    UnstructuredGrid(GridDefinitionTemplate3_101),
    Missing,
    Unknown(u16),
}
