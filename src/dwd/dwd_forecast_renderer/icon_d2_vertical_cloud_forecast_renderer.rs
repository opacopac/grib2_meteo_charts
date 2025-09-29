use crate::dwd::common::icon_d2_model_config::IconD2ModelConfig;
use crate::dwd::dwd_file_reader::icon_d2_clc_reader::IconD2ClcReader;
use crate::dwd::dwd_file_reader::icon_d2_hhl_reader::IconD2HhlReader;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::meteo_chart::meteo_layer::meteo_vertical_cloud_layer::MeteoVerticalCloudLayer;
use crate::metobin::vertical_cloud_metobin::VerticalCloudMeteobin;
use log::info;


pub struct IconD2VerticalCloudForecastRenderer;


impl IconD2VerticalCloudForecastRenderer {
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
                let clc_grids = IconD2ClcReader::read_clc_grids(&fc_step, &vertical_levels)?;
                let layer = MeteoVerticalCloudLayer::new(&hhl_grids, clc_grids);

                // meteobin
                let _ = VerticalCloudMeteobin::create_meteobin_file(&layer, forecast_run, step);

                Ok(())
            })
    }
}
