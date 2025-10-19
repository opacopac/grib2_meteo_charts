use crate::dwd::dwd_file_reader::icon_d2_clct_mod_reader::IconD2ClctModReader;
use crate::dwd::dwd_file_reader::icon_d2_tot_prec_reader::IconD2TotPrecReader;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;


pub struct IconD2CloudPrecipReader;


impl IconD2CloudPrecipReader {
    pub fn read_layer_from_files(
        fc_run: &MeteoForecastRun2,
        fc_step_clct: &MeteoForecastRun2Step,
        fc_step_precip0: &MeteoForecastRun2Step,
        fc_step_precip1: &MeteoForecastRun2Step,
    ) -> Result<MeteoCloudPrecipLayer, Grib2Error> {
        let grid_clct = IconD2ClctModReader::read_grid_from_file(fc_run, fc_step_clct)?;
        let grid_precip0 = IconD2TotPrecReader::read_grid_from_file(fc_run, fc_step_precip0)?;
        let grid_precip1 = IconD2TotPrecReader::read_grid_from_file(fc_run, fc_step_precip1)?;

        let layer = MeteoCloudPrecipLayer::new(grid_clct, grid_precip0, grid_precip1)?;

        Ok(layer)
    }
}
