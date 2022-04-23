use crate::geo::lat_lon::LatLon;
use crate::grib2::common::scale_factor_value::ScaleFactorValue;
use crate::grib2::section3::resolution_and_component_flags::ResolutionAndComponentFlags;
use crate::grib2::section3::scanning_mode_flags::ScanningModeFlags;
use crate::grib2::section3::shape_of_earth::ShapeOfEarth;

#[derive(Debug)]
pub struct GridDefinitionTemplate3_0 {
    pub shape_of_earth: ShapeOfEarth,
    pub spherical_earth_radius: ScaleFactorValue,
    pub oblated_spheroid_earth_major_axis: ScaleFactorValue,
    pub oblated_spheroid_earth_minor_axis: ScaleFactorValue,
    pub number_of_points_along_parallel: u32,
    pub number_of_points_along_meridian: u32,
    pub initial_production_domain_basic_angle: u32,
    pub initial_production_domain_subdivision: u32,
    pub first_grid_point: LatLon,
    pub resolution_component_flags: ResolutionAndComponentFlags,
    pub last_grid_point: LatLon,
    pub i_direction_increment: f32,
    pub j_direction_increment: f32,
    pub scanning_mode_flags: ScanningModeFlags
}


impl GridDefinitionTemplate3_0 {
    pub fn new(
        shape_of_earth: ShapeOfEarth,
        spherical_earth_radius: ScaleFactorValue,
        oblated_spheroid_earth_major_axis: ScaleFactorValue,
        oblated_spheroid_earth_minor_axis: ScaleFactorValue,
        number_of_points_along_parallel: u32,
        number_of_points_along_meridian: u32,
        initial_production_domain_basic_angle: u32,
        initial_production_domain_subdivision: u32,
        first_grid_point: LatLon,
        resolution_component_flags: ResolutionAndComponentFlags,
        last_grid_point: LatLon,
        i_direction_increment: f32,
        j_direction_increment: f32,
        scanning_mode_flags: ScanningModeFlags
    ) -> GridDefinitionTemplate3_0 {
        return GridDefinitionTemplate3_0 {
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
            last_grid_point,
            i_direction_increment,
            j_direction_increment,
            scanning_mode_flags
        }
    }
}
