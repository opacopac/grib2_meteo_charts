use crate::grib2::section3::grid_definition_template_3_0::GridDefinitionTemplate3_0;

pub enum GridDefinitionTemplate {
    LatitudeLongitude(GridDefinitionTemplate3_0),
    Missing,
    Unknown(u16),
}
