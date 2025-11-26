use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
use crate::meteo_swiss::file_reader::icon_ch_clct_reader::IconChClctReader;
use crate::meteo_swiss::file_reader::icon_ch_tot_prec_reader::IconChTotPrecReader;


pub struct IconChCloudPrecipReader;


impl IconChCloudPrecipReader {
    pub fn read_layer_from_files(
        clct_step: &MeteoForecastRunStep,
        prec0_step: &MeteoForecastRunStep,
        prec1_step: &MeteoForecastRunStep,
        unstructured_grid: &UnstructuredGrid,
    ) -> Result<MeteoCloudPrecipLayer, Grib2Error> {
        let clct_grid = IconChClctReader::read_grid_from_file(clct_step.get_file_url(), &unstructured_grid)?;
        let tot_prec0 = IconChTotPrecReader::read_grid_from_file(prec0_step.get_file_url(), &unstructured_grid)?;
        let tot_prec1 = IconChTotPrecReader::read_grid_from_file(prec1_step.get_file_url(), &unstructured_grid)?;

        let layer = MeteoCloudPrecipLayer::new(clct_grid.clone(), tot_prec0, tot_prec1)?;

        Ok(layer)
    }
}
