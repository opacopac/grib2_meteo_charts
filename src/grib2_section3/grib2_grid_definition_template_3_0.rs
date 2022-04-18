use crate::grib2_section3::grib2_shape_of_earth::Grib2ShapeOfEarth;

#[derive(PartialEq, Debug)]
pub struct Grib2gridDefinitionTemplate3_0 {
    pub shape_of_earth: Grib2ShapeOfEarth,
    pub spherical_earth_radius_scale_factor: u8,
    pub spherical_earth_radius_scale_value: u8
}


impl Grib2gridDefinitionTemplate3_0 {
    pub fn new(
        shape_of_earth: Grib2ShapeOfEarth,
        spherical_earth_radius_scale_factor: u8,
        spherical_earth_radius_scale_value: u8
    ) -> Grib2gridDefinitionTemplate3_0 {
        return Grib2gridDefinitionTemplate3_0 {
            shape_of_earth,
            spherical_earth_radius_scale_factor,
            spherical_earth_radius_scale_value
        }
    }
}
