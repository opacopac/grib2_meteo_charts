use crate::dwd::common::icon_d2_model_config::IconD2ModelConfig;
use crate::dwd::dwd_file_reader::icon_d2_hhl_reader::IconD2HhlReader;
use crate::dwd::dwd_file_reader::icon_d2_u_reader::IconD2UReader;
use crate::dwd::dwd_file_reader::icon_d2_v_reader::IconD2VReader;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::meteo_chart::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;
use crate::metobin::vertical_wind_metobin::VerticalWindMeteobin;
use log::info;


pub struct IconD2VerticalWindForecastRenderer;


impl IconD2VerticalWindForecastRenderer {
    pub fn render(
        forecast_run: &DwdForecastRun,
        step_filter: &Vec<usize>,
    ) -> Result<(), ForecastRendererError> {
        let vertical_levels = IconD2ModelConfig::get_vertical_level_range();
        let hhl_grids = IconD2HhlReader::read_hhl_grids(forecast_run, &vertical_levels)?;

        DwdForecastStep::get_step_range()
            .try_for_each(|step| {
                if !step_filter.is_empty() && !step_filter.contains(&step) {
                    return Ok(());
                }

                info!("creating vertical cloud charts, time step {}", step);
                let fc_step = DwdForecastStep::new_from_run(forecast_run, step);
                let u_grids = IconD2UReader::read_u_grids(&fc_step, &vertical_levels)?;
                let v_grids = IconD2VReader::read_v_grids(&fc_step, &vertical_levels)?;
                let layer = MeteoVerticalWindLayer::new(hhl_grids.clone(), u_grids, v_grids);

                // meteobin
                let _ = VerticalWindMeteobin::create_meteobin_file(&layer, forecast_run, step)?;

                Ok(())
            })
    }
}
