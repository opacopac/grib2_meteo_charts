use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grid::unstructured_grid::UnstructuredGrid;
use crate::grid::unstructured_value_grid::UnstructuredValueGrid;

pub struct UnstructuredGridConverter;

impl UnstructuredGridConverter {
    pub fn create(
        grib2_doc: &Grib2Document,
        missing_value: f32,
        coordinates: Vec<LatLon>,
        dimensions: (usize, usize),
        lat_lon_extent: LatLonExtent,
        max_coord_dist_deg: f32,
    ) -> Result<UnstructuredValueGrid<f32>, Grib2Error> {
        let unstructured_values = grib2_doc.calculate_data_points(missing_value, |x| x as f32)?;

        if coordinates.len() != unstructured_values.len() {
            return Err(Grib2Error::InvalidData(
                "number of lat/lon and grib2 data points don't match".to_string(),
            ));
        }

        let mut grid = UnstructuredGrid::new(
            dimensions,
            lat_lon_extent,
            coordinates,
        );
        grid.calc_coord_dist_lookup_map(max_coord_dist_deg);

        let value_grid = UnstructuredValueGrid::new(unstructured_values, missing_value, grid);

        Ok(value_grid)
    }
}
