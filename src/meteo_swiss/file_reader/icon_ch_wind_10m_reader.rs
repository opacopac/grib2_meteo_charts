use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_chart::forecast_renderer::meteo_chart_error::MeteoChartError;
use crate::meteo_chart::meteo_layer::meteo_wind_10m_layer::MeteoWind10mLayer;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use crate::meteo_swiss::file_reader::icon_ch_wind_u_10m_reader::IconChWindU10mReader;
use crate::meteo_swiss::file_reader::icon_ch_wind_v_10m_reader::IconChWindV10mReader;
use crate::meteo_swiss::file_reader::icon_ch_wind_vmax_10m_reader::IconChWindVmax10mReader;


pub struct IconChWind10mReader;


impl IconChWind10mReader {
    pub fn read_layer_from_files(
        u_step: &MeteoForecastRun2Step,
        v_step: &MeteoForecastRun2Step,
        v_max_step: &MeteoForecastRun2Step,
        unstructured_grid: &UnstructuredGrid,
    ) -> Result<MeteoWind10mLayer, MeteoChartError> {
        let grid_u = IconChWindU10mReader::read_grid_from_file(u_step.get_file_url(), unstructured_grid)?;
        let grid_v = IconChWindV10mReader::read_grid_from_file(v_step.get_file_url(), unstructured_grid)?;
        let grid_gusts = IconChWindVmax10mReader::read_grid_from_file(v_max_step.get_file_url(), unstructured_grid)?;

        let layer = MeteoWind10mLayer::new(grid_u, grid_v, Some(grid_gusts))?;

        Ok(layer)
    }
}
