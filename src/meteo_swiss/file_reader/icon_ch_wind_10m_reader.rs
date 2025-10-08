use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_chart::meteo_layer::meteo_wind_10m_layer::MeteoWind10mLayer;
use crate::meteo_swiss::common::meteo_swiss_error::MeteoSwissError;
use crate::meteo_swiss::file_reader::icon_ch_wind_u_10m_reader::IconChWindU10mReader;
use crate::meteo_swiss::file_reader::icon_ch_wind_v_10m_reader::IconChWindV10mReader;
use crate::meteo_swiss::file_reader::icon_ch_wind_vmax_10m_reader::IconChWindVmax10mReader;


pub struct IconChWind10mReader;


impl IconChWind10mReader {
    pub fn read_layer_from_files(
        file_url_u: &str,
        file_url_v: &str,
        file_url_v_max: &str,
        unstructured_grid: &UnstructuredGrid,
    ) -> Result<MeteoWind10mLayer, MeteoSwissError> {
        let grid_u = IconChWindU10mReader::read_grid_from_file(file_url_u, unstructured_grid)?;
        let grid_v = IconChWindV10mReader::read_grid_from_file(file_url_v, unstructured_grid)?;
        let grid_gusts = IconChWindVmax10mReader::read_grid_from_file(file_url_v_max, unstructured_grid)?;

        let layer = MeteoWind10mLayer::new(grid_u, grid_v, Some(grid_gusts))?;

        Ok(layer)
    }
}
