use crate::geo::common::lat_lon::LatLon;
use crate::geo::common::lat_lon_extent::LatLonExtent;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::lat_lon_value_grid_interpolator::LatLonValueGridInterpolator;
use crate::meteo_chart::meteo_layer::meteo_layer::MeteoLayer;
use crate::meteo_chart::meteo_layer::meteo_layer_error::MeteoLayerError;


pub struct MeteoCloudPrecipLayer {
    layer_type: MeteoLayer,
    cloud_value_grid: LatLonValueGrid<f32>,
    precip0_value_grid: LatLonValueGrid<f32>,
    precip1_value_grid: LatLonValueGrid<f32>,
}


impl MeteoCloudPrecipLayer {
    pub fn new(
        cloud_value_grid: LatLonValueGrid<f32>,
        precip0_value_grid: LatLonValueGrid<f32>,
        precip1_value_grid: LatLonValueGrid<f32>,
    ) -> Result<MeteoCloudPrecipLayer, MeteoLayerError> {
        if cloud_value_grid.get_grid_dimensions() != precip0_value_grid.get_grid_dimensions() ||
            precip0_value_grid.get_grid_dimensions() != precip1_value_grid.get_grid_dimensions() {
            return Err(MeteoLayerError::InvalidData("grids have different dimensions".to_string()));
        }

        if cloud_value_grid.get_grid_lat_lon_extent() != precip0_value_grid.get_grid_lat_lon_extent() ||
            precip0_value_grid.get_grid_lat_lon_extent() != precip1_value_grid.get_grid_lat_lon_extent() {
            return Err(MeteoLayerError::InvalidData("grids have different lat lon extents".to_string()));
        }

        let layer = MeteoCloudPrecipLayer {
            layer_type: MeteoLayer::CloudPrecip,
            cloud_value_grid,
            precip0_value_grid,
            precip1_value_grid,
        };

        Ok(layer)
    }
    
    
    pub fn get_type(&self) -> &MeteoLayer {
        &self.layer_type
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
