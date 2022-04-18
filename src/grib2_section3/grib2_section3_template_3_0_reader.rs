use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_common::angle_reader::AngleReader;
use crate::grib2_common::scale_factor_value_reader::ScaleFactorValueReader;
use crate::grib2_section3::grib2_grid_definition_template_3_0::Grib2gridDefinitionTemplate3_0;
use crate::grib2_section3::grib2_resolution_and_component_flags::Grib2ResolutionAndComponentFlags;
use crate::grib2_section3::grib2_scanning_mode_flags::Grib2ScanningModeFlags;
use crate::grib2_section3::grib2_shape_of_earth::Grib2ShapeOfEarth;

pub struct Grib2Section3Template3_0Reader;


impl Grib2Section3Template3_0Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Grib2gridDefinitionTemplate3_0, Box<dyn Error>> {
        let shape_of_earth = Grib2Section3Template3_0Reader::read_shape_of_earth(reader)?;
        let spherical_earth_radius = ScaleFactorValueReader::read(reader)?;
        let oblated_spheroid_earth_major_axis = ScaleFactorValueReader::read(reader)?;
        let oblated_spheroid_earth_minor_axis = ScaleFactorValueReader::read(reader)?;
        let number_of_points_along_parallel = reader.read_u32::<BigEndian>()?;
        let number_of_points_along_meridian = reader.read_u32::<BigEndian>()?;
        let initial_production_domain_basic_angle = reader.read_u32::<BigEndian>()?;
        let initial_production_domain_subdivision = reader.read_u32::<BigEndian>()?;
        let first_grid_point = AngleReader::read_lat_lon(reader)?;
        let resolution_component_flags = Grib2Section3Template3_0Reader::read_resolution_and_component_flags(reader)?;
        let last_grid_point = AngleReader::read_lat_lon(reader)?;
        let i_direction_increment = AngleReader::read_angle(reader)?;
        let j_direction_increment = AngleReader::read_angle(reader)?;
        let scanning_mode_flags = Grib2Section3Template3_0Reader::read_scanning_mode_flags(reader)?;
        let tpl_3_0 = Grib2gridDefinitionTemplate3_0::new(
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
        );

        return Ok(tpl_3_0);
    }


    fn read_shape_of_earth(reader: &mut BufReader<File>) -> Result<Grib2ShapeOfEarth, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let shape_of_earth = match value {
            6 => Grib2ShapeOfEarth::SphericalRadius6371229,
            255 => Grib2ShapeOfEarth::Missing,
            _ => Grib2ShapeOfEarth::Unknown(value)
        };

        return Ok(shape_of_earth);
    }


    fn read_resolution_and_component_flags(reader: &mut BufReader<File>) -> Result<Grib2ResolutionAndComponentFlags, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let flags = Grib2ResolutionAndComponentFlags::new(
            (value & 0b00000100) == 0,
            (value & 0b00001000) == 0,
            (value & 0b00010000) == 0
        );

        return Ok(flags);
    }


    fn read_scanning_mode_flags(reader: &mut BufReader<File>) -> Result<Grib2ScanningModeFlags, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let flags = Grib2ScanningModeFlags::new(
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
