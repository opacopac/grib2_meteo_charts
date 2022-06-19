use std::fs;
use std::io::Read;

use bzip2::read::BzDecoder;

use crate::chart::cloud_precip_chart_renderer::CloudPrecipChartRenderer;
use crate::chart::wind_chart_renderer::WindChartRenderer;
use crate::dwd_chart_builder::icon_d2_forecast_run_finder::IconD2ForecastRunFinder;
use crate::dwd_files::icon_d2_file::DWD_DATE_FORMAT;
use crate::dwd_files::icon_d2_file_clct_mod::IconD2FileClctMod;
use crate::dwd_files::icon_d2_file_tot_prec::IconD2FileTotPrec;
use crate::dwd_files::icon_d2_file_u_10m::IconD2FileU10m;
use crate::dwd_files::icon_d2_file_v_10m::IconD2FileV10m;
use crate::dwd_forecast_runs::icon_d2_forecast_run::IconD2ForecastRun;
use crate::dwd_forecast_runs::icon_d2_forecast_step::IconD2ForecastStep;
use crate::dwd_layer::dwd_cloud_precip_layer::DwdCloudPrecipLayer;
use crate::dwd_layer::dwd_wind_layer::DwdWindLayer;
use crate::grib2::document::grib2_document_reader::Grib2DocumentReader;
use crate::grid::regular_grid_converter::RegularGridConverter;
use crate::imaging::drawable::Drawable;

pub struct IconD2ChartBuilder;


impl IconD2ChartBuilder {
    pub fn create_latest_dwd_forecasts() {
        let latest_run = IconD2ForecastRunFinder::find_latest_forecast_run().unwrap(); // TODO

        Self::create_weather_map_tiles(&latest_run);
        //Self::create_wind_map_tiles(&latest_run);
    }


    fn create_weather_map_tiles(forecast_run: &IconD2ForecastRun) {
        //for step in IconD2ForecastStep::get_step_range() {
        for step in 2..=10 {
            println!("time step {}", step);

            let fc_step = IconD2ForecastStep::new_from_run(forecast_run, step);
            let fc_previous_step = IconD2ForecastStep::new_from_run(forecast_run, step - 1);

            let clct_file = IconD2FileClctMod::get_file_url(&fc_step);
            let mut clct_reader = Self::get_file_reader(&clct_file);
            let clct_doc = Grib2DocumentReader::read_stream(&mut clct_reader).unwrap();
            let clct_grid = RegularGridConverter::create(&clct_doc, -1.0).unwrap();

            let precip_file0 = IconD2FileTotPrec::get_file_url(&fc_previous_step);
            let mut precip_reader0 = Self::get_file_reader(&precip_file0);
            let precip_doc0 = Grib2DocumentReader::read_stream(&mut precip_reader0).unwrap();
            let precip_grid0 = RegularGridConverter::create(&precip_doc0, -1.0).unwrap();

            let precip_file1 = IconD2FileTotPrec::get_file_url(&fc_step);
            let mut precip_reader1 = Self::get_file_reader(&precip_file1);
            let precip_doc1 = Grib2DocumentReader::read_stream(&mut precip_reader1).unwrap();
            let precip_grid1 = RegularGridConverter::create(&precip_doc1, -1.0).unwrap();

            let layer = DwdCloudPrecipLayer::new(clct_grid, precip_grid0, precip_grid1).unwrap();

            let _ = CloudPrecipChartRenderer::render_map_tiles(
                &layer,
                (0, 7),
                |tile: &Drawable, zoom: u32, x: u32, y: u32| Self::save_tile_step(tile, zoom, x, y, &fc_step)
            );
        }
    }


    fn create_wind_map_tiles(forecast_run: &IconD2ForecastRun) {
        //for step in IconD2ForecastStep::get_step_range() {
        for step in 0..=10 {
            println!("time step {}", step);

            let fc_step = IconD2ForecastStep::new_from_run(forecast_run, step);

            let wind_u_file = IconD2FileU10m::get_file_url(&fc_step);
            let mut wind_u_reader = Self::get_file_reader(&wind_u_file);
            let wind_u_doc = Grib2DocumentReader::read_stream(&mut wind_u_reader).unwrap();
            let wind_u_grid = RegularGridConverter::create(&wind_u_doc, -1.0).unwrap();

            let wind_v_file = IconD2FileV10m::get_file_url(&fc_step);
            let mut wind_v_reader = Self::get_file_reader(&wind_v_file);
            let wind_v_doc = Grib2DocumentReader::read_stream(&mut wind_v_reader).unwrap();
            let wind_v_grid0 = RegularGridConverter::create(&wind_v_doc, -1.0).unwrap();

            let layer = DwdWindLayer::new(wind_u_grid, wind_v_grid0, None).unwrap();

            let _ = WindChartRenderer::render_map_tiles(
                &layer,
                (0, 7),
                |tile: &Drawable, zoom: u32, x: u32, y: u32| Self::save_tile_step(tile, zoom, x, y, &fc_step)
            );
        }
    }


    fn get_file_reader(filename: &str) -> impl Read {
        let response_result = ureq::get(filename).call().unwrap(); // TODO
        let reader = response_result.into_reader();
        let bz_decoder = BzDecoder::new(reader);

        return bz_decoder;
    }


    fn save_tile_step(
        tile: &Drawable,
        zoom: u32,
        x: u32,
        y: u32,
        fc_step: &IconD2ForecastStep
    ) {
        let path = format!(
            "{}/{}{}/{:03}/{}/{}",
            "./output", // TODO
            fc_step.run.date.format(DWD_DATE_FORMAT),
            fc_step.run.run_name.get_name(),
            fc_step.step,
            zoom,
            x
        );
        fs::create_dir_all(&path).unwrap();

        let filename = format!("{}/{}.png", &path, y);
        let _result = tile.safe_image(&filename);
    }
}
