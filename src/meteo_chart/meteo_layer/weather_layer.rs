use crate::geo::common::lat_lon::LatLon;
use crate::geo::common::lat_lon_extent::LatLonExtent;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::meteo_chart::meteo_layer::meteo_layer_error::MeteoLayerError;
use crate::meteo_chart::meteo_layer::meteo_layer_type::MeteoLayerType;
use crate::meteo_chart::meteo_layer::weather_interpretation::WeatherInterpretation;


pub struct WeatherLayer {
    layer_type: MeteoLayerType,
    clct_grid: LatLonValueGrid<f32>,
    ceiling_grid: LatLonValueGrid<f32>,
    ww_grid: Option<LatLonValueGrid<f32>>,
}


impl WeatherLayer {
    pub fn new(
        clct_grid: LatLonValueGrid<f32>,
        ceiling_grid: LatLonValueGrid<f32>,
        ww_grid: Option<LatLonValueGrid<f32>>,
    ) -> Result<WeatherLayer, MeteoLayerError> {
        if clct_grid.get_grid_dimensions() != ceiling_grid.get_grid_dimensions() { // TODO: also check ww_grid if present
            return Err(MeteoLayerError::InvalidData("grids have different dimensions".to_string()));
        }

        if clct_grid.get_grid_lat_lon_extent() != ceiling_grid.get_grid_lat_lon_extent() { // TODO: also check ww_grid if present
            return Err(MeteoLayerError::InvalidData("grids have different lat lon extents".to_string()));
        }

        let layer = WeatherLayer {
            layer_type: MeteoLayerType::Weather,
            clct_grid,
            ceiling_grid,
            ww_grid,
        };

        Ok(layer)
    }


    pub fn get_type(&self) -> &MeteoLayerType {
        &self.layer_type
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        self.clct_grid.get_grid_dimensions()
    }


    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        self.clct_grid.get_grid_lat_lon_extent()
    }


    pub fn get_ceiling_by_xy(&self, x: usize, y: usize) -> Option<f32> {
        self.ceiling_grid.get_value_by_xy(x, y)
    }


    pub fn get_ceiling_by_lat_lon(&self, pos: &LatLon) -> Option<f32> {
        self.ceiling_grid.get_value_by_lat_lon(pos)
    }


    pub fn get_clct_by_xy(&self, x: usize, y: usize) -> Option<f32> {
        self.clct_grid.get_value_by_xy(x, y)
    }


    pub fn get_clct_by_lat_lon(&self, pos: &LatLon) -> Option<f32> {
        self.clct_grid.get_value_by_lat_lon(pos)
    }


    pub fn get_ww_by_xy(&self, x: usize, y: usize) -> Option<WeatherInterpretation> {
        match &self.ww_grid {
            None => None, // TODO: return clct based interpretation
            Some(ww_grid2) => ww_grid2
                .get_value_by_xy(x, y)
                .map(|v| WeatherInterpretation::from_value(v as u8))
        }
    }


    pub fn get_ww_by_lat_lon(&self, pos: &LatLon) -> Option<WeatherInterpretation> {
        match &self.ww_grid {
            None => None, // TODO: return clct based interpretation
            Some(ww_grid2) => ww_grid2
                .get_value_by_lat_lon(pos)
                .map(|v| WeatherInterpretation::from_value(v as u8))
        }
    }
}
