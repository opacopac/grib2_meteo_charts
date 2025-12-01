use crate::dwd::dwd_file_reader::dwd_icon_clct_mod_reader::DwdIconClctModReader;
use crate::dwd::dwd_file_reader::icon_d2_tot_prec_reader::IconD2TotPrecReader;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct IconD2CloudPrecipReader;


impl IconD2CloudPrecipReader {
    pub fn read_layer(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> Result<MeteoCloudPrecipLayer, Grib2Error> {
        let grid_clct = DwdIconClctModReader::read_grid_from_file(fc_run, fc_step)?;
        let grid_precip0 = IconD2TotPrecReader::read_grid_from_file(fc_run, fc_step)?;
        let grid_precip1 = IconD2TotPrecReader::read_grid_from_file(fc_run, fc_step)?;

        let layer = MeteoCloudPrecipLayer::new(grid_clct, grid_precip0, grid_precip1)?;

        Ok(layer)
    }
}
