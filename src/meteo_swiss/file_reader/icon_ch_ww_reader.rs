use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::weather_layer::WeatherLayer;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
use crate::meteo_swiss::file_reader::icon_ch_ceiling_reader::IconChCeilingReader;
use crate::meteo_swiss::file_reader::icon_ch_clct_reader::IconChClctReader;


pub struct IconChWwReader;


impl IconChWwReader {
    pub fn read_layer(
        fc_step: &MeteoForecastRunStep,
        all_clct_steps: &[MeteoForecastRunStep],
        all_ceiling_steps: &[MeteoForecastRunStep],
        unstructured_grid: &UnstructuredGrid,
    ) -> Result<WeatherLayer, Grib2Error> {
        let step_nr = fc_step.get_step_nr();
        let clct_step = MeteoForecastRunStep::get_step_by_nr(&all_clct_steps, step_nr).unwrap(); // TODO
        let ceiling_step = MeteoForecastRunStep::get_step_by_nr(&all_ceiling_steps, step_nr).unwrap(); // TODO

        let file_url_clct = &clct_step.get_file_url();
        let file_url_ceiling = &ceiling_step.get_file_url();
        let clct_grid = IconChClctReader::read_grid_from_file(file_url_clct, &unstructured_grid)?;
        let ceiling_grid = IconChCeilingReader::read_grid_from_file(file_url_ceiling, &unstructured_grid)?;

        let ww_layer = WeatherLayer::new(clct_grid, ceiling_grid, None)?;

        Ok(ww_layer)
    }
}
