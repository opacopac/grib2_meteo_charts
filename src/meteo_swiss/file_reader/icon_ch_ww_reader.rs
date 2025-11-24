use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::weather_layer::WeatherLayer;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use crate::meteo_swiss::file_reader::icon_ch_ceiling_reader::IconChCeilingReader;
use crate::meteo_swiss::file_reader::icon_ch_clct_reader::IconChClctReader;


pub struct IconChWwReader;


impl IconChWwReader {
    pub fn read_layer_from_files(
        clct_step: &MeteoForecastRun2Step,
        ceiling_step: &MeteoForecastRun2Step,
        unstructured_grid: &UnstructuredGrid,
    ) -> Result<WeatherLayer, Grib2Error> {
        let file_url_clct = &clct_step.get_file_url();
        let file_url_ceiling = &ceiling_step.get_file_url();
        let clct_grid = IconChClctReader::read_grid_from_file(file_url_clct, &unstructured_grid)?;
        let ceiling_grid = IconChCeilingReader::read_grid_from_file(file_url_ceiling, &unstructured_grid)?;

        let ww_layer = WeatherLayer::new(clct_grid, ceiling_grid, None)?;

        Ok(ww_layer)
    }
}
