use crate::dwd::common::dwd_error::DwdError;
use crate::dwd::dwd_files::icon_d2_file_to_grid_converter::IconD2FileToGridConverter;
use crate::dwd::dwd_files::icon_d2_file_v_10m::IconD2FileV10m;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;

pub struct IconD2V10mReader;

impl IconD2V10mReader {
    pub fn read_grid_from_file(fc_step: &DwdForecastStep) -> Result<LatLonValueGrid<f32>, DwdError> {
        let url = IconD2FileV10m::get_file_url(&fc_step);

        return IconD2FileToGridConverter::read_grid_from_file(&url);
    }
}
