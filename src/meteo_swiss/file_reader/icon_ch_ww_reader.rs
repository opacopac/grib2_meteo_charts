use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::meteo_layer::weather_layer::WeatherLayer;
use crate::meteo_swiss::file_reader::icon_ch_ceiling_reader::IconChCeilingReader;
use crate::meteo_swiss::file_reader::icon_ch_clct_reader::IconChClctReader;


pub struct IconChWwReader;


impl IconChWwReader {
    pub fn read_layer_from_files(
        file_url_clct: &str,
        file_url_ceiling: &str,
        unstructured_grid: &UnstructuredGrid,
    ) -> Result<WeatherLayer, Grib2Error> {
        let clct_grid = IconChClctReader::read_grid_from_file(file_url_clct, &unstructured_grid)?;
        let ceiling_grid = IconChCeilingReader::read_grid_from_file(file_url_ceiling, &unstructured_grid)?;

        let ww_layer = WeatherLayer::new(clct_grid, ceiling_grid, None)?;

        Ok(ww_layer)
    }
}
