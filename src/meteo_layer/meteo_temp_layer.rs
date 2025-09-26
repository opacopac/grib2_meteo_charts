use crate::geo::common::lat_lon::LatLon;
use crate::geo::common::lat_lon_extent::LatLonExtent;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::meteo_layer::meteo_layer_error::MeteoLayerError;

pub struct MeteoTempLayer {
    temp_value_grid: LatLonValueGrid<f32>,
}


impl MeteoTempLayer {
    pub fn new(
        temp_value_grid: LatLonValueGrid<f32>
    ) -> Result<MeteoTempLayer, MeteoLayerError> {
        let layer = MeteoTempLayer { temp_value_grid };

        Ok(layer)
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        self.temp_value_grid.get_grid_dimensions()
    }


    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        self.temp_value_grid.get_grid_lat_lon_extent()
    }


    pub fn get_temp_by_xy(&self, x: usize, y: usize) -> Option<f32> {
        self.temp_value_grid.get_value_by_xy(x, y)
    }


    pub fn get_temp_by_lat_lon(&self, pos: &LatLon) -> Option<f32> {
        self.temp_value_grid.get_value_by_lat_lon(pos)
    }
}
