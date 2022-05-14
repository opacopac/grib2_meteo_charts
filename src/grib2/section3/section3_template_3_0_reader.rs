use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::angle_reader::AngleReader;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::common::scale_factor_value_reader::ScaleFactorValueReader;
use crate::grib2::section3::grid_definition_template_3_0::GridDefinitionTemplate3_0;
use crate::grib2::section3::resolution_and_component_flags::ResolutionAndComponentFlags;
use crate::grib2::section3::scanning_mode_flags::ScanningModeFlags;
use crate::grib2::section3::shape_of_earth_reader::ShapeOfEarthReader;

pub struct Section3Template3_0Reader;


impl Section3Template3_0Reader {
    pub fn read<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<GridDefinitionTemplate3_0, Grib2Error> {
        let shape_of_earth = ShapeOfEarthReader::read(reader)?;
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


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::grib2::section3::section3_template_3_0_reader::Section3Template3_0Reader;
    use crate::grib2::section3::shape_of_earth::ShapeOfEarth;

    #[test]
    fn it_correctly_parses_template_3_0() {
        let mut reader = BufReader::new(Cursor::new([
            0x06, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0x00, 0x00, 0x04, 0xBF, 0x00, 0x00, 0x02, 0xEA, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
            0x02, 0x92, 0xDF, 0xE0, 0x15, 0x39, 0x0B, 0x60, 0x30, 0x03, 0x76, 0x3B, 0x00, 0x01, 0x36, 0x5D,
            0x20, 0x00, 0x00, 0x4E, 0x20, 0x00, 0x00, 0x4E, 0x20, 0x40
        ]));

        let result = Section3Template3_0Reader::read(&mut reader);
        assert!(result.is_ok());

        let tpl30 = result.unwrap();
        assert_eq!(ShapeOfEarth::SphericalRadius6371229, tpl30.shape_of_earth);
        assert_eq!(255, tpl30.spherical_earth_radius.factor);
        assert_eq!(4294967295, tpl30.spherical_earth_radius.value);
        assert_eq!(255, tpl30.oblated_spheroid_earth_major_axis.factor);
        assert_eq!(4294967295, tpl30.oblated_spheroid_earth_major_axis.value);
        assert_eq!(255, tpl30.oblated_spheroid_earth_minor_axis.factor);
        assert_eq!(4294967295, tpl30.oblated_spheroid_earth_minor_axis.value);
        assert_eq!(1215, tpl30.number_of_points_along_parallel);
        assert_eq!(746, tpl30.number_of_points_along_meridian);
        assert_eq!(0, tpl30.initial_production_domain_basic_angle);
        assert_eq!(4294967295, tpl30.initial_production_domain_subdivision);
        assert_eq!(43.180000, tpl30.first_grid_point_lat);
        assert_eq!(356.060000, tpl30.first_grid_point_lon);
        assert_eq!(true, tpl30.resolution_component_flags.i_direction_increments_not_given);
        assert_eq!(true, tpl30.resolution_component_flags.j_direction_increments_not_given);
        assert_eq!(false, tpl30.resolution_component_flags.u_v_relative_to_e_n);
        assert_eq!(58.080000, tpl30.last_grid_point_lat);
        assert_eq!(20.340000, tpl30.last_grid_point_lon);
        assert_eq!(0.020000, tpl30.i_direction_increment);
        assert_eq!(0.020000, tpl30.j_direction_increment);
        assert_eq!(true, tpl30.scanning_mode_flags.scan_direction_first_row_i_is_positive);
        assert_eq!(true, tpl30.scanning_mode_flags.scan_direction_first_row_j_is_negative);
        assert_eq!(true, tpl30.scanning_mode_flags.adjacent_points_in_i_direction_consecutive);
        assert_eq!(true, tpl30.scanning_mode_flags.all_rows_same_scan_direction);
        assert_eq!(true, tpl30.scanning_mode_flags.odd_rows_offset_in_i_direction);
        assert_eq!(true, tpl30.scanning_mode_flags.even_rows_offset_in_i_direction);
        assert_eq!(false, tpl30.scanning_mode_flags.points_not_offset_in_j_direction);
        assert_eq!(true, tpl30.scanning_mode_flags.rows_have_ni_points_cols_have_nj_points);
    }
}
