use std::fs::File;
use std::io::{BufWriter, Write};

use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::chart::wind_chart_renderer::WindChartRenderer;
use crate::dwd_chart_builder::icon_d2_chart_builder_helper::IconD2ChartBuilderHelper;
use crate::dwd_files::icon_d2_file_u_10m::IconD2FileU10m;
use crate::dwd_files::icon_d2_file_v_10m::IconD2FileV10m;
use crate::dwd_files::icon_d2_file_vmax_10m::IconD2FileVmax10m;
use crate::dwd_forecast_runs::dwd_forecast_run::DwdForecastRun;
use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;
use crate::dwd_layer::dwd_wind_layer::DwdWindLayer;
use crate::imaging::drawable::Drawable;
use crate::metobin::dwd_wind_metobin::DwdWindMeteobin;

pub struct IconD2WindChartBuilder;

const WIND_LAYER: &str = "wind";


impl IconD2WindChartBuilder {
    pub fn create(forecast_run: &DwdForecastRun) {
        DwdForecastStep::get_step_range()
            .into_par_iter()
            .for_each(|step| {
                info!("creating wind charts, time step {}", step);

                let fc_step = DwdForecastStep::new_from_run(forecast_run, step);

                let wind_u_grid = IconD2FileU10m::read_grid_from_file(&fc_step).unwrap(); // TODO
                let wind_v_grid = IconD2FileV10m::read_grid_from_file(&fc_step).unwrap();
                let wind_v_max_grid = IconD2FileVmax10m::read_grid_from_file(&fc_step).unwrap();

                let layer = DwdWindLayer::new(wind_u_grid, wind_v_grid, Some(wind_v_max_grid)).unwrap();

                // map tiles
                let _ = WindChartRenderer::render_map_tiles(
                    &layer,
                    (0, 7),
                    |tile: &Drawable, zoom: u32, x: u32, y: u32| IconD2ChartBuilderHelper::save_tile_step(tile, zoom, x, y, WIND_LAYER, &fc_step)
                );

                // meteobin
                let wind_bin = DwdWindMeteobin::new(layer);
                let data = wind_bin.create_bin_values();
                let filename = format!(
                    "{}WIND_D2.meteobin",
                    IconD2ChartBuilderHelper::get_output_path(&fc_step, WIND_LAYER),
                );
                let mut file = BufWriter::new(File::create(&filename).expect("Unable to create wind meteobin file"));
                let _ = file.write_all(&data);
            });
    }
}
