use derive_new::new;

use crate::grib2::common::scale_factor_value::ScaleFactorValue;
use crate::grib2::section3::resolution_and_component_flags::ResolutionAndComponentFlags;
use crate::grib2::section3::scanning_mode_flags::ScanningModeFlags;
use crate::grib2::section3::shape_of_earth::ShapeOfEarth;

#[derive(Debug, new)]
pub struct GridDefinitionTemplate3_0 {
    pub shape_of_earth: ShapeOfEarth,
    pub spherical_earth_radius: ScaleFactorValue,
    pub oblated_spheroid_earth_major_axis: ScaleFactorValue,
    pub oblated_spheroid_earth_minor_axis: ScaleFactorValue,
    pub number_of_points_along_parallel: u32,
    pub number_of_points_along_meridian: u32,
    pub initial_production_domain_basic_angle: u32,
    pub initial_production_domain_subdivision: u32,
    pub first_grid_point_lat: f32,
    pub first_grid_point_lon: f32,
    pub resolution_component_flags: ResolutionAndComponentFlags,
    pub last_grid_point_lat: f32,
    pub last_grid_point_lon: f32,
    pub i_direction_increment: f32,
    pub j_direction_increment: f32,
    pub scanning_mode_flags: ScanningModeFlags
}
