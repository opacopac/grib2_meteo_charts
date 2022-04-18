use crate::grib2_section3::grib2_shape_of_earth::Grib2ShapeOfEarth;

#[derive(PartialEq, Debug)]
pub struct Grib2gridDefinitionTemplate3_0 {
    pub shape_of_earth: Grib2ShapeOfEarth,
    pub spherical_earth_radius_scale_factor: u8,
    pub spherical_earth_radius_scale_value: u16,
    pub oblated_spheroid_earth_major_axis_scale_factor: u8,
    pub oblated_spheroid_earth_major_axis_scale_value: u16,
    pub oblated_spheroid_earth_minor_axis_scale_factor: u8,
    pub oblated_spheroid_earth_minor_axis_scale_value: u16,
    pub number_of_points_along_parallel: u16,
    pub number_of_points_along_meridian: u16
}


impl Grib2gridDefinitionTemplate3_0 {
    pub fn new(
        shape_of_earth: Grib2ShapeOfEarth,
        spherical_earth_radius_scale_factor: u8,
        spherical_earth_radius_scale_value: u16,
        oblated_spheroid_earth_major_axis_scale_factor: u8,
        oblated_spheroid_earth_major_axis_scale_value: u16,
        oblated_spheroid_earth_minor_axis_scale_factor: u8,
        oblated_spheroid_earth_minor_axis_scale_value: u16,
        number_of_points_along_parallel: u16,
        number_of_points_along_meridian: u16
    ) -> Grib2gridDefinitionTemplate3_0 {
        return Grib2gridDefinitionTemplate3_0 {
            shape_of_earth,
            spherical_earth_radius_scale_factor,
            spherical_earth_radius_scale_value,
            oblated_spheroid_earth_major_axis_scale_factor,
            oblated_spheroid_earth_major_axis_scale_value,
            oblated_spheroid_earth_minor_axis_scale_factor,
            oblated_spheroid_earth_minor_axis_scale_value,
            number_of_points_along_parallel,
            number_of_points_along_meridian
        }
    }
}