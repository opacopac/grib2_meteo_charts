use crate::geo::common::lat_lon::LatLon;
use crate::geo::common::lat_lon_extent::LatLonExtent;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::meteo_chart::meteo_layer::meteo_layer_error::MeteoLayerError;

pub struct MeteoWindLayer {
    zonal_value_grid: LatLonValueGrid<f32>,
    meridional_value_grid: LatLonValueGrid<f32>,
    gusts_value_grid: Option<LatLonValueGrid<f32>>,
}


impl MeteoWindLayer {
    pub fn new(
        zonal_value_grid: LatLonValueGrid<f32>,
        meridional_value_grid: LatLonValueGrid<f32>,
        gusts_value_grid: Option<LatLonValueGrid<f32>>,
    ) -> Result<MeteoWindLayer, MeteoLayerError> {
        if zonal_value_grid.get_grid_dimensions() != meridional_value_grid.get_grid_dimensions() {
            return Err(MeteoLayerError::InvalidData("grids have different dimensions".to_string()));
        }

        if zonal_value_grid.get_grid_lat_lon_extent() != meridional_value_grid.get_grid_lat_lon_extent() {
            return Err(MeteoLayerError::InvalidData("grids have different lat lon extents".to_string()));
        }

        if let Some(gusts_grid) = &gusts_value_grid {
            if gusts_grid.get_grid_dimensions() != zonal_value_grid.get_grid_dimensions() {
                return Err(MeteoLayerError::InvalidData("grids have different dimensions".to_string()));
            }

            if gusts_grid.get_grid_lat_lon_extent() != zonal_value_grid.get_grid_lat_lon_extent() {
                return Err(MeteoLayerError::InvalidData("grids have different lat lon extents".to_string()));
            }
        }

        let layer = MeteoWindLayer { zonal_value_grid, meridional_value_grid, gusts_value_grid };

        Ok(layer)
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        self.zonal_value_grid.get_grid_dimensions()
    }


    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        self.zonal_value_grid.get_grid_lat_lon_extent()
    }


    pub fn get_wind_speed_e_n_by_xy(&self, x: usize, y: usize) -> Option<(f32, f32)> {
        let value_e = self.zonal_value_grid.get_value_by_xy(x, y);
        let value_n = self.meridional_value_grid.get_value_by_xy(x, y);

        match (value_e, value_n) {
            (Some(val_e), Some(val_n)) => Some((val_e, val_n)),
            _ => None
        }
    }


    pub fn get_wind_speed_e_n_by_lat_lon(&self, pos: &LatLon) -> Option<(f32, f32)> {
        let value_e = self.zonal_value_grid.get_value_by_lat_lon(pos);
        let value_n = self.meridional_value_grid.get_value_by_lat_lon(pos);

        match (value_e, value_n) {
            (Some(val_e), Some(val_n)) => Some((val_e, val_n)),
            _ => None
        }
    }


    pub fn get_wind_speed_tot_xy(&self, x: usize, y: usize) -> Option<f32> {
        self.get_wind_speed_e_n_by_xy(x, y)
            .map(|(value_e, value_n)| (value_e * value_e + value_n * value_n).sqrt())
    }


    pub fn get_wind_speed_tot_by_lat_lon(&self, pos: &LatLon) -> Option<f32> {
        self.get_wind_speed_e_n_by_lat_lon(pos)
            .map(|(value_e, value_n)| (value_e * value_e + value_n * value_n).sqrt())
    }


    pub fn get_wind_speed_dir_by_xy(&self, x: usize, y: usize) -> Option<(f32, f32)> {
        self.get_wind_speed_e_n_by_xy(x, y)
            .map(|(value_e, value_n)| (
                (value_e * value_e + value_n * value_n).sqrt(),
                value_n.atan2(value_e).to_degrees()
            ))
    }


    pub fn get_gusts_by_xy(&self, x: usize, y: usize) -> Option<f32> {
        match &self.gusts_value_grid {
            Some(gusts_grid) => gusts_grid.get_value_by_xy(x, y),
            _ => None
        }
    }


    pub fn get_gusts_by_lat_lon(&self, pos: &LatLon) -> Option<f32> {
        match &self.gusts_value_grid {
            Some(gusts_grid) => gusts_grid.get_value_by_lat_lon(pos),
            _ => None
        }
    }
}
