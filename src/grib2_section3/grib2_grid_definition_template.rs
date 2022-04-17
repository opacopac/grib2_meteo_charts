use crate::grib2_section3::grib2_grid_definition_template_3_101::Grib2gridDefinitionTemplate3101;


pub enum Grib2GridDefinitionTemplate {
    UnstructuredGrid(Grib2gridDefinitionTemplate3101),
    Missing(),
    Unknown(u16),
}
