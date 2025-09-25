use crate::dwd::common::dwd_error::DwdError;
use crate::dwd::dwd_files::icon_d2_file_clct_mod::IconD2FileClctMod;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;

pub struct IconD2ClctModReader;

impl IconD2ClctModReader {
    pub fn read_grid_from_file(fc_step: &DwdForecastStep) -> Result<LatLonValueGrid<f32>, DwdError> {
        let url = IconD2FileClctMod::get_file_url(&fc_step);
        let missing_value = -1.0;
        let grid = FileToGridConverter::read_rectangular_grid_from_file(&url, missing_value)?;
        
        Ok(grid)
    }
}
