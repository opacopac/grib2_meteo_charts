use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::angle_reader::AngleReader;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::common::scale_factor_value_reader::ScaleFactorValueReader;
use crate::grib2::section3::grid_definition_template_3_0::GridDefinitionTemplate3_0;
use crate::grib2::section3::resolution_and_component_flags::ResolutionAndComponentFlags;
use crate::grib2::section3::scanning_mode_flags::ScanningModeFlags;
use crate::grib2::section3::shape_of_earth::ShapeOfEarth;

pub struct Section3Template3_0Reader;


impl Section3Template3_0Reader {
    pub fn read<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<GridDefinitionTemplate3_0, Grib2Error> {
        let shape_of_earth = Section3Template3_0Reader::read_shape_of_earth(reader)?;
        let spherical_earth_radius = ScaleFactorValueReader::read(reader)?;
        let oblated_spheroid_earth_major_axis = ScaleFactorValueReader::read(reader)?;
        let oblated_spheroid_earth_minor_axis = ScaleFactorValueReader::read(reader)?;
        let number_of_points_along_parallel = reader.read_u32::<BigEndian>()?;
        let number_of_points_along_meridian = reader.read_u32::<BigEndian>()?;
        let initial_production_domain_basic_angle = reader.read_u32::<BigEndian>()?;
        let initial_production_domain_subdivision = reader.read_u32::<BigEndian>()?;
        let first_grid_point = AngleReader::read_lat_lon(reader)?;
        let resolution_component_flags = Section3Template3_0Reader::read_resolution_and_component_flags(reader)?;
        let last_grid_point = AngleReader::read_lat_lon(reader)?;
        let i_direction_increment = AngleReader::read_angle(reader)?;
        let j_direction_increment = AngleReader::read_angle(reader)?;
        let scanning_mode_flags = Section3Template3_0Reader::read_scanning_mode_flags(reader)?;
        let tpl_3_0 = GridDefinitionTemplate3_0::new(
            shape_of_earth,
            spherical_earth_radius,
            oblated_spheroid_earth_major_axis,
            oblated_spheroid_earth_minor_axis,
            number_of_points_along_parallel,
            number_of_points_along_meridian,
            initial_production_domain_basic_angle,
            initial_production_domain_subdivision,
            first_grid_point.0,
            first_grid_point.1,
            resolution_component_flags,
            last_grid_point.0,
            last_grid_point.1,
            i_direction_increment,
            j_direction_increment,
            scanning_mode_flags
        );

        return Ok(tpl_3_0);
    }


    fn read_shape_of_earth<T: Read>(reader: &mut BufReader<T>) -> Result<ShapeOfEarth, Grib2Error> {
        let value = reader.read_u8()?;
        let shape_of_earth = match value {
            6 => ShapeOfEarth::SphericalRadius6371229,
            255 => ShapeOfEarth::Missing,
            _ => ShapeOfEarth::Unknown(value)
        };

        return Ok(shape_of_earth);
    }


    fn read_resolution_and_component_flags<T: Read>(reader: &mut BufReader<T>) -> Result<ResolutionAndComponentFlags, Grib2Error> {
        let value = reader.read_u8()?;
        let flags = ResolutionAndComponentFlags::new(
            (value & 0b00000100) == 0,
            (value & 0b00001000) == 0,
            (value & 0b00010000) == 0
        );

        return Ok(flags);
    }


    fn read_scanning_mode_flags<T: Read>(reader: &mut BufReader<T>) -> Result<ScanningModeFlags, Grib2Error> {
        let value = reader.read_u8()?;
        let flags = ScanningModeFlags::new(
            (value & 0b00000001) == 0,
            (value & 0b00000010) == 0,
            (value & 0b00000100) == 0,
            (value & 0b00001000) == 0,
            (value & 0b00010000) == 0,
            (value & 0b00100000) == 0,
            (value & 0b01000000) == 0,
            (value & 0b10000000) == 0
        );

        return Ok(flags);
    }
}
