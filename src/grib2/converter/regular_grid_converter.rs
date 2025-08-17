use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section3::grid_definition_template::GridDefinitionTemplate;
use crate::grid::grid_value_type::GridValueType;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;

pub struct RegularGridConverter;


impl RegularGridConverter {
    pub fn create(doc: &Grib2Document, missing_value: f32) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        return Self::create_and_transform(doc, missing_value, |x| x as f32);
    }


    pub fn create_and_transform<T: GridValueType>(
        doc: &Grib2Document,
        missing_value: T,
        transform_fn: fn(f32) -> T,
    ) -> Result<LatLonValueGrid<T>, Grib2Error> {
        let values = doc.calculate_data_points(missing_value, transform_fn)?;
        let (dimensions, lat_lon_extent) = Self::get_dimensions_and_extent(doc)?;
        let grid = LatLonValueGrid::new(
            values,
            missing_value,
            dimensions,
            lat_lon_extent,
        );

        return Ok(grid);
    }


    // TODO: re-project if 1st point lat/lon > last point lat/lon
    fn get_dimensions_and_extent(document: &Grib2Document) -> Result<((usize, usize), LatLonExtent), Grib2Error> {
        return match &document.section3.grid_definition_template {
            GridDefinitionTemplate::LatitudeLongitude(tpl) => {
                Ok((
                    (
                        tpl.number_of_points_along_parallel as usize,
                        tpl.number_of_points_along_meridian as usize
                    ),
                    LatLonExtent::new(
                        LatLon::new(tpl.first_grid_point_lat, tpl.first_grid_point_lon),
                        LatLon::new(tpl.last_grid_point_lat, tpl.last_grid_point_lon),
                    )
                ))
            }
            _ => Err(Grib2Error::InvalidData("invalid grid definition template, only LatLonGrid is supported.".to_string()))
        };
    }
}
