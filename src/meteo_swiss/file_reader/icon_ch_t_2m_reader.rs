use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_chart::meteo_layer::meteo_temp_2m_layer::MeteoTemp2mLayer;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct IconChT2mReader;


impl IconChT2mReader {
    const MISSING_VALUE: f32 = -1.0;


    pub fn read_layer(
        fc_step: &MeteoForecastRunStep,
        all_t2m_steps: &[MeteoForecastRunStep],
        unstructured_grid: &UnstructuredGrid,
    ) -> Result<MeteoTemp2mLayer, Grib2Error> {
        let step_nr = fc_step.get_step_nr();
        let t2m_step = MeteoForecastRunStep::get_step_by_nr(&all_t2m_steps, step_nr).unwrap(); // TODO

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


    fn transform_values(value: f32) -> f32 {
        value
    }
}
