use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;


pub struct MeteoTempLayer {
    temp_value_grid: LatLonValueGrid<f32>,
}


impl MeteoTempLayer {
    pub fn new(
        temp_value_grid: LatLonValueGrid<f32>
    ) -> Result<MeteoTempLayer, Grib2Error> {
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
