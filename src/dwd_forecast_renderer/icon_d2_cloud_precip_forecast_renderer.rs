use std::fs::File;
use std::io::{BufWriter, Write};

use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::chart::cloud_precip_chart_renderer::CloudPrecipChartRenderer;
use crate::dwd_forecast_renderer::icon_d2_forecast_renderer_helper::IconD2ForecastRendererHelper;
use crate::dwd::dwd_files::icon_d2_file_ceiling::IconD2FileCeiling;
use crate::dwd::dwd_files::icon_d2_file_clct_mod::IconD2FileClctMod;
use crate::dwd::dwd_files::icon_d2_file_tot_prec::IconD2FileTotPrec;
use crate::dwd::dwd_files::icon_d2_file_ww::IconD2FileWw;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::dwd_layer::dwd_cloud_precip_layer::DwdCloudPrecipLayer;
use crate::dwd_layer::dwd_weather_layer::DwdWeatherLayer;
use crate::imaging::drawable::Drawable;
use crate::metobin::dwd_weather_metobin::DwdWeatherMeteoBin;

pub struct IconD2CloudPrecipRenderer;


const WEATHER_LAYER: &str = "clct_precip";


impl IconD2CloudPrecipRenderer {
    pub fn create(forecast_run: &DwdForecastRun) {
        DwdForecastStep::get_step_range()
            .into_par_iter()
            .for_each(|step| {
                info!("creating weather charts, time step {}", step);

                let fc_step = DwdForecastStep::new_from_run(forecast_run, step);
                let fc_previous_step = DwdForecastStep::new_from_run(forecast_run, step - 1);

                // map tiles
                let clct_grid = IconD2FileClctMod::read_grid_from_file(&fc_step).unwrap(); // TODO
                let precip_grid0 = IconD2FileTotPrec::read_grid_from_file(&fc_previous_step).unwrap();
                let precip_grid1 = IconD2FileTotPrec::read_grid_from_file(&fc_step).unwrap();

                let layer = DwdCloudPrecipLayer::new(clct_grid, precip_grid0, precip_grid1).unwrap();

                let _ = CloudPrecipChartRenderer::render_map_tiles(
                    &layer,
                    (0, 7),
                    |tile: &Drawable, zoom: u32, x: u32, y: u32| IconD2ForecastRendererHelper::save_tile_step(tile, zoom, x, y, WEATHER_LAYER, &fc_step)
                );


                // meteobin
                let ww_grid = IconD2FileWw::read_grid_from_file(&fc_step).unwrap();
                let ceiling_grid = IconD2FileCeiling::read_grid_from_file(&fc_step).unwrap();

                let weather_layer = DwdWeatherLayer::new(ww_grid, ceiling_grid).unwrap();
                let weather_bin = DwdWeatherMeteoBin::new(weather_layer);
                let data = weather_bin.create_bin_values();
                let filename = format!(
                    "{}WW_D2.meteobin",
                    IconD2ForecastRendererHelper::get_output_path(&fc_step, WEATHER_LAYER),
                );
                let mut file = BufWriter::new(File::create(&filename).expect("Unable to create file"));
                let _ = file.write_all(&data);
            });
    }
}
