use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_chart::forecast_renderer::meteo_chart_error::MeteoChartError;
use crate::meteo_chart::meteo_layer::meteo_wind_10m_layer::MeteoWind10mLayer;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
use crate::meteo_swiss::file_reader::icon_ch_wind_u_10m_reader::IconChWindU10mReader;
use crate::meteo_swiss::file_reader::icon_ch_wind_v_10m_reader::IconChWindV10mReader;
use crate::meteo_swiss::file_reader::icon_ch_wind_vmax_10m_reader::IconChWindVmax10mReader;


pub struct IconChWind10mReader;


impl IconChWind10mReader {
    pub fn read_layer(
        fc_step: &MeteoForecastRunStep,
        all_u10m_steps: &[MeteoForecastRunStep],
        all_v10m_steps: &[MeteoForecastRunStep],
        all_vmax10m_steps: &[MeteoForecastRunStep],
        unstructured_grid: &UnstructuredGrid,
    ) -> Result<MeteoWind10mLayer, MeteoChartError> {
        let step_nr = fc_step.get_step_nr();
        let u10m_step = MeteoForecastRunStep::get_step_by_nr(&all_u10m_steps, step_nr)?;
        let v10m_step = MeteoForecastRunStep::get_step_by_nr(&all_v10m_steps, step_nr)?;
        let vmax10m_step = MeteoForecastRunStep::get_step_by_nr(&all_vmax10m_steps, step_nr)?;

        let grid_u = IconChWindU10mReader::read_grid_from_file(u10m_step.get_file_url(), unstructured_grid)?;
        let grid_v = IconChWindV10mReader::read_grid_from_file(v10m_step.get_file_url(), unstructured_grid)?;
        let grid_gusts = IconChWindVmax10mReader::read_grid_from_file(vmax10m_step.get_file_url(), unstructured_grid)?;

        let layer = MeteoWind10mLayer::new(grid_u, grid_v, Some(grid_gusts))?;

        Ok(layer)
    }
}
