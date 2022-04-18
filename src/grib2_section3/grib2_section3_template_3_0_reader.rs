use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_section3::grib2_grid_definition_template_3_0::Grib2gridDefinitionTemplate3_0;
use crate::grib2_section3::grib2_shape_of_earth::Grib2ShapeOfEarth;

pub struct Grib2Section3Template3_0Reader;


impl Grib2Section3Template3_0Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Grib2gridDefinitionTemplate3_0, Box<dyn Error>> {
        let shape_of_earth = Grib2Section3Template3_0Reader::read_shape_of_earth(reader)?;
        let spherical_earth_radius_scale_factor = reader.read_u8()?;
        let spherical_earth_radius_scale_value = reader.read_u16::<BigEndian>()?;
        let oblated_spheroid_earth_major_axis_scale_factor = reader.read_u8()?;
        let oblated_spheroid_earth_major_axis_scale_value = reader.read_u16::<BigEndian>()?;
        let oblated_spheroid_earth_minor_axis_scale_factor = reader.read_u8()?;
        let oblated_spheroid_earth_minor_axis_scale_value = reader.read_u16::<BigEndian>()?;
        let number_of_points_along_parallel = reader.read_u16::<BigEndian>()?;
        let number_of_points_along_meridian = reader.read_u16::<BigEndian>()?;
        let tpl_3_0 = Grib2gridDefinitionTemplate3_0::new(
            shape_of_earth,
            spherical_earth_radius_scale_factor,
            spherical_earth_radius_scale_value,
            oblated_spheroid_earth_major_axis_scale_factor,
            oblated_spheroid_earth_major_axis_scale_value,
            oblated_spheroid_earth_minor_axis_scale_factor,
            oblated_spheroid_earth_minor_axis_scale_value,
            number_of_points_along_parallel,
            number_of_points_along_meridian
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
}
