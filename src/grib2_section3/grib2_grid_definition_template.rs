use crate::grib2_section3::grib2_grid_definition_template_3_0::Grib2gridDefinitionTemplate3_0;

#[derive(PartialEq, Debug)]
pub enum Grib2GridDefinitionTemplate {
    LatLon(Grib2gridDefinitionTemplate3_0),
    Missing,
    Unknown(u16),
}
