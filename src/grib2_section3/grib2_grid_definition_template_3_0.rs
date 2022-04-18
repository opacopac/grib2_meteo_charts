use crate::grib2_common::lat_lon::LatLon;
use crate::grib2_common::scale_factor_value::ScaleFactorValue;
use crate::grib2_section3::grib2_resolution_and_component_flags::Grib2ResolutionAndComponentFlags;
use crate::grib2_section3::grib2_shape_of_earth::Grib2ShapeOfEarth;

pub struct Grib2gridDefinitionTemplate3_0 {
    pub shape_of_earth: Grib2ShapeOfEarth,
    pub spherical_earth_radius: ScaleFactorValue,
    pub oblated_spheroid_earth_major_axis: ScaleFactorValue,
    pub oblated_spheroid_earth_minor_axis: ScaleFactorValue,
    pub number_of_points_along_parallel: u32,
    pub number_of_points_along_meridian: u32,
    pub initial_production_domain_basic_angle: u32,
    pub initial_production_domain_subdivision: u32,
    pub first_grid_point: LatLon,
    pub resolution_component_flags: Grib2ResolutionAndComponentFlags,
    pub last_grid_point: LatLon,
}


impl Grib2gridDefinitionTemplate3_0 {
    pub fn new(
        shape_of_earth: Grib2ShapeOfEarth,
        spherical_earth_radius: ScaleFactorValue,
        oblated_spheroid_earth_major_axis: ScaleFactorValue,
        oblated_spheroid_earth_minor_axis: ScaleFactorValue,
        number_of_points_along_parallel: u32,
        number_of_points_along_meridian: u32,
        initial_production_domain_basic_angle: u32,
        initial_production_domain_subdivision: u32,
        first_grid_point: LatLon,
        resolution_component_flags: Grib2ResolutionAndComponentFlags,
        last_grid_point: LatLon
    ) -> Grib2gridDefinitionTemplate3_0 {
        return Grib2gridDefinitionTemplate3_0 {
            shape_of_earth,
            spherical_earth_radius,
            oblated_spheroid_earth_major_axis,
            oblated_spheroid_earth_minor_axis,
            number_of_points_along_parallel,
            number_of_points_along_meridian,
            initial_production_domain_basic_angle,
            initial_production_domain_subdivision,
            first_grid_point,
            resolution_component_flags,
            last_grid_point
        }
    }
}
