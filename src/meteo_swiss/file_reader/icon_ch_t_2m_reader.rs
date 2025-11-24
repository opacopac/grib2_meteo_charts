use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_chart::meteo_layer::meteo_temp_2m_layer::MeteoTemp2mLayer;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;

pub struct IconChT2mReader;


impl IconChT2mReader {
    const MISSING_VALUE: f32 = -1.0;


    pub fn read_layer_from_file(t2m_step: &MeteoForecastRun2Step, unstructured_grid: &UnstructuredGrid) -> Result<MeteoTemp2mLayer, Grib2Error> {
        let regular_grid = Self::read_grid_from_file(t2m_step.get_file_url(), unstructured_grid)?;
        let layer = MeteoTemp2mLayer::new(regular_grid);

        Ok(layer)
    }


    fn read_grid_from_file(file_url: &str, unstructured_grid: &UnstructuredGrid) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let regular_grid = FileToGridConverter::read_unstructured_grid_from_file_and_transform(
            file_url,
            Self::MISSING_VALUE,
            Self::transform_values,
            unstructured_grid,
        )?;

        Ok(regular_grid)
    }


    pub fn transform_values(value: f32) -> f32 {
        value
    }
}
