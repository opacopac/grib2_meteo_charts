use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::imaging::drawable::Drawable;
use crate::meteo_chart::cloud_precip_chart_renderer::CloudPrecipChartRenderer;
use crate::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;
use crate::meteo_layer::weather_layer::WeatherLayer;
use crate::meteo_swiss::file_reader::icon_ch_ceiling_reader::IconChCeilingReader;
use crate::meteo_swiss::file_reader::icon_ch_clct_reader::IconChClctReader;
use crate::meteo_swiss::file_reader::icon_ch_tot_prec_reader::IconChTotPrecReader;
use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_renderer_helper::IconCh1ForecastRendererHelper;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::meteo_swiss::common::meteo_swiss_error::MeteoSwissError;
use crate::metobin::precip_metobin::PrecipMeteoBin;
use crate::metobin::weather_metobin::WeatherMeteoBin;
use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::{BufWriter, Write};


pub struct IconCh1CloudPrecipRenderer;


const WEATHER_LAYER: &str = "clct_precip";


impl IconCh1CloudPrecipRenderer {
    pub fn render(
        forecast_run_clct: &IconChForecastRun,
        forecast_run_tot_prec: &IconChForecastRun,
        forecast_run_ceiling: &IconChForecastRun,
        unstructured_grid: &UnstructuredGrid,
        step_filter: &Vec<usize>,
    ) -> Result<(), MeteoSwissError> {
        forecast_run_clct.get_step_range()
            .into_par_iter()
            .try_for_each(|step_idx| {
                if !step_filter.is_empty() && !step_filter.contains(&step_idx) {
                    return Ok(());
                }

                info!("creating weather charts, time step {}", step_idx);

                let fc_step_clct = &forecast_run_clct.steps[step_idx];
                let fc_step_tot_prec = &forecast_run_tot_prec.steps[step_idx];
                let fc_previous_step_tot_prec = &forecast_run_tot_prec.steps[step_idx - 1];

                let clct_grid = IconChClctReader::read_grid_from_file(&fc_step_clct.href, &unstructured_grid)?;
                let precip_grid0 = IconChTotPrecReader::read_grid_from_file(&fc_previous_step_tot_prec.href, &unstructured_grid)?;
                let precip_grid1 = IconChTotPrecReader::read_grid_from_file(&fc_step_tot_prec.href, &unstructured_grid)?;

                let layer = MeteoCloudPrecipLayer::new(clct_grid.clone(), precip_grid0, precip_grid1)?;

                // map tiles
                let _ = CloudPrecipChartRenderer::render_map_tiles(
                    &layer,
                    IconCh1ForecastRendererHelper::get_zoom_range(),
                    |tile: &Drawable, zoom: u32, x: u32, y: u32| IconCh1ForecastRendererHelper::save_tile_step(
                        tile, zoom, x, y, WEATHER_LAYER, forecast_run_clct, step_idx,
                    ),
                );

                // precip meteobin
                let precip_bin_data = PrecipMeteoBin::create_bin_values(&layer);
                let precip_filename = format!(
                    "{}PRECIP.meteobin",
                    IconCh1ForecastRendererHelper::get_output_path(forecast_run_clct, step_idx, WEATHER_LAYER),
                );
                let mut precip_file = BufWriter::new(File::create(&precip_filename).expect("Unable to create file"));
                let _ = precip_file.write_all(&precip_bin_data);


                // ww meteobin
                let fc_step_ceiling = &forecast_run_ceiling.steps[step_idx];
                let ceiling_grid = IconChCeilingReader::read_grid_from_file(&fc_step_ceiling.href, &unstructured_grid)?;

                let weather_layer = WeatherLayer::new(clct_grid, ceiling_grid, None)?;
                let ww_bin_data = WeatherMeteoBin::create_bin_values(&weather_layer);
                let ww_filename = format!(
                    "{}WW.meteobin",
                    IconCh1ForecastRendererHelper::get_output_path(forecast_run_clct, step_idx, WEATHER_LAYER),
                );
                let mut ww_file = BufWriter::new(File::create(&ww_filename).expect("Unable to create file"));
                let _ = ww_file.write_all(&ww_bin_data);

                Ok(())
            })
    }
}
