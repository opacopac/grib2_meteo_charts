use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grid::lat_lon_value_grid_interpolator::LatLonValueGridInterpolator;


pub struct MeteoCloudPrecipLayer {
    cloud_value_grid: LatLonValueGrid<f32>,
    precip0_value_grid: LatLonValueGrid<f32>,
    precip1_value_grid: LatLonValueGrid<f32>,
}


impl MeteoCloudPrecipLayer {
    pub fn new(
        cloud_value_grid: LatLonValueGrid<f32>,
        precip0_value_grid: LatLonValueGrid<f32>,
        precip1_value_grid: LatLonValueGrid<f32>,
    ) -> Result<MeteoCloudPrecipLayer, Grib2Error> {
        if cloud_value_grid.get_grid_dimensions() != precip0_value_grid.get_grid_dimensions() ||
            precip0_value_grid.get_grid_dimensions() != precip1_value_grid.get_grid_dimensions() {
            return Err(Grib2Error::InvalidData("grids have different dimensions".to_string()));
        }

        if cloud_value_grid.get_grid_lat_lon_extent() != precip0_value_grid.get_grid_lat_lon_extent() ||
            precip0_value_grid.get_grid_lat_lon_extent() != precip1_value_grid.get_grid_lat_lon_extent() {
            return Err(Grib2Error::InvalidData("grids have different lat lon extents".to_string()));
        }

        let layer = MeteoCloudPrecipLayer { cloud_value_grid, precip0_value_grid, precip1_value_grid };

        Ok(layer)
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        self.cloud_value_grid.get_grid_dimensions()
    }


    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        self.cloud_value_grid.get_grid_lat_lon_extent()
    }


    pub fn get_cloud_and_precip_by_xy(&self, x: usize, y: usize) -> Option<(f32, f32)> {
        let cloud_value = self.cloud_value_grid.get_value_by_xy(x, y).unwrap_or(0.0);
        let precip0_value = self.precip0_value_grid.get_value_by_xy(x, y).unwrap_or(0.0);
        let precip1_value = self.precip1_value_grid.get_value_by_xy(x, y).unwrap_or(0.0);
        let precip_delta_value = precip1_value - precip0_value;

        Some((cloud_value, precip_delta_value))
    }


    pub fn get_cloud_and_precip_by_lat_lon(&self, pos: &LatLon) -> Option<(f32, f32)> {
        let cloud_value = LatLonValueGridInterpolator::interpolate(&self.cloud_value_grid, pos).unwrap_or(0.0);
        let precip0_value = LatLonValueGridInterpolator::interpolate(&self.precip0_value_grid, pos).unwrap_or(0.0);
        let precip1_value = LatLonValueGridInterpolator::interpolate(&self.precip1_value_grid, pos).unwrap_or(0.0);
        let precip_delta_value = precip1_value - precip0_value;

        Some((cloud_value, precip_delta_value))
    }
}
