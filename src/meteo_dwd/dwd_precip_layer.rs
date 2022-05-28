use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::meteo_dwd::lat_lon_value_grid::LatLonValueGrid;

pub struct DwdPrecipLayer {
    value_grid: LatLonValueGrid<f32>
}


impl DwdPrecipLayer {
    pub fn new(value_grid: LatLonValueGrid<f32>) -> DwdPrecipLayer {
        return DwdPrecipLayer { value_grid };
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        return self.value_grid.get_grid_dimensions();
    }


    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        return self.value_grid.get_grid_lat_lon_extent();
    }


    pub fn get_total_precipitation_by_xy(&self, x: usize, y: usize) -> Option<f32> {
        return self.value_grid.get_value_by_xy(x, y);
    }


    pub fn get_total_precipitation_by_lat_lon(&self, pos: &LatLon) -> Option<f32> {
        return self.value_grid.get_value_by_lat_lon(pos);
    }
}
