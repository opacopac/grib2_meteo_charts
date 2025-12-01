use crate::dwd::dwd_file_reader::dwd_icon_clct_mod_reader::DwdIconClctModReader;
use crate::dwd::dwd_file_reader::dwd_icon_tot_prec_reader::DwdIconTotPrecReader;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct DwdIconCloudPrecipReader;


impl DwdIconCloudPrecipReader {
    pub fn read_layer(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
        all_clct_steps: &[MeteoForecastRunStep],
        all_tot_precip_steps: &[MeteoForecastRunStep],
    ) -> Result<MeteoCloudPrecipLayer, Grib2Error> {
        let step_nr = fc_step.get_step_nr();
        let clct_step = MeteoForecastRunStep::get_step_by_nr(&all_clct_steps, step_nr).unwrap(); // TODO
        let prec0_step = MeteoForecastRunStep::get_step_by_nr(&all_tot_precip_steps, step_nr - 1).unwrap(); // TODO
        let prec1_step = MeteoForecastRunStep::get_step_by_nr(&all_tot_precip_steps, step_nr).unwrap(); // TODO

        let grid_clct = DwdIconClctModReader::read_grid_from_file(fc_run, clct_step)?;
        let grid_precip0 = DwdIconTotPrecReader::read_grid_from_file(fc_run, prec0_step)?;
        let grid_precip1 = DwdIconTotPrecReader::read_grid_from_file(fc_run, prec1_step)?;

        let layer = MeteoCloudPrecipLayer::new(grid_clct, grid_precip0, grid_precip1)?;

        Ok(layer)
    }
}
