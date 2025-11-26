use crate::dwd::dwd_file_reader::icon_d2_u_10m_reader::IconD2U10mReader;
use crate::dwd::dwd_file_reader::icon_d2_v_10m_reader::IconD2V10mReader;
use crate::dwd::dwd_file_reader::icon_d2_vmax_10m_reader::IconD2Vmax10mReader;
use crate::meteo_chart::forecast_renderer::meteo_chart_error::MeteoChartError;
use crate::meteo_chart::meteo_layer::meteo_wind_10m_layer::MeteoWind10mLayer;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct IconD2Wind10mReader;


impl IconD2Wind10mReader {
    pub fn read_layer(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> Result<MeteoWind10mLayer, MeteoChartError> {
        let grid_u = IconD2U10mReader::read_grid_from_file(fc_run, fc_step)?;
        let grid_v = IconD2V10mReader::read_grid_from_file(fc_run, fc_step)?;
        let grid_gusts = IconD2Vmax10mReader::read_grid_from_file(fc_run, fc_step)?;

        let layer = MeteoWind10mLayer::new(grid_u, grid_v, Some(grid_gusts))?;

        Ok(layer)
    }


    pub fn read_layer_from_files(
        fc_run: &MeteoForecastRun,
        fc_step_u10m: &MeteoForecastRunStep,
        fc_step_v10m: &MeteoForecastRunStep,
        fc_step_vmax10m: &MeteoForecastRunStep,
    ) -> Result<MeteoWind10mLayer, MeteoChartError> {
        let grid_u = IconD2U10mReader::read_grid_from_file(fc_run, fc_step_u10m)?;
        let grid_v = IconD2V10mReader::read_grid_from_file(fc_run, fc_step_v10m)?;
        let grid_gusts = IconD2Vmax10mReader::read_grid_from_file(fc_run, fc_step_vmax10m)?;

        let layer = MeteoWind10mLayer::new(grid_u, grid_v, Some(grid_gusts))?;

        Ok(layer)
    }
}
