use std::fs;
use std::fs::File;
use std::io::{BufWriter, Read, Write};

use bzip2::read::BzDecoder;
use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::chart::cloud_precip_chart_renderer::CloudPrecipChartRenderer;
use crate::chart::wind_chart_renderer::WindChartRenderer;
use crate::dwd_chart_builder::icon_d2_forecast_run_finder::IconD2ForecastRunFinder;
use crate::dwd_files::icon_d2_file::DWD_DATE_FORMAT;
use crate::dwd_files::icon_d2_file_ceiling::IconD2FileCeiling;
use crate::dwd_files::icon_d2_file_clct_mod::IconD2FileClctMod;
use crate::dwd_files::icon_d2_file_tot_prec::IconD2FileTotPrec;
use crate::dwd_files::icon_d2_file_u_10m::IconD2FileU10m;
use crate::dwd_files::icon_d2_file_v_10m::IconD2FileV10m;
use crate::dwd_files::icon_d2_file_vmax_10m::IconD2FileVmax10m;
use crate::dwd_files::icon_d2_file_ww::IconD2FileWw;
use crate::dwd_forecast_runs::dwd_forecast_run::DwdForecastRun;
use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;
use crate::dwd_layer::dwd_cloud_precip_layer::DwdCloudPrecipLayer;
use crate::dwd_layer::dwd_weather_layer::DwdWeatherLayer;
use crate::dwd_layer::dwd_wind_layer::DwdWindLayer;
use crate::grib2::document::grib2_document_reader::Grib2DocumentReader;
use crate::grid::regular_grid_converter::RegularGridConverter;
use crate::imaging::drawable::Drawable;
use crate::metobin::dwd_weather_metobin::DwdWeatherMeteoBin;
use crate::metobin::dwd_wind_metobin::DwdWindMeteobin;

pub struct IconD2ChartBuilder;


const FORECAST_BASE_DIR: &str = "./output/icon-d2/";
const WEATHER_LAYER: &str = "clct_precip";
const WIND_LAYER: &str = "wind";


impl IconD2ChartBuilder {
    pub fn create_latest_dwd_forecasts() {
        let latest_run = IconD2ForecastRunFinder::find_latest_forecast_run().unwrap(); // TODO
        info!("latest run found: {:?}", &latest_run);

        Self::create_weather_map_tiles(&latest_run);
        Self::create_wind_charts(&latest_run);
    }


    fn create_weather_map_tiles(forecast_run: &DwdForecastRun) {
        DwdForecastStep::get_step_range().into_par_iter().for_each(|step| {
            info!("creating weather charts, time step {}", step);

            let fc_step = DwdForecastStep::new_from_run(forecast_run, step);
            let fc_previous_step = DwdForecastStep::new_from_run(forecast_run, step - 1);

            // map tiles
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
                |tile: &Drawable, zoom: u32, x: u32, y: u32| Self::save_tile_step(tile, zoom, x, y, WEATHER_LAYER, &fc_step)
            );


            // meteobin
            let ww_file = IconD2FileWw::get_file_url(&fc_step);
            let mut ww_reader = Self::get_file_reader(&ww_file);
            let ww_doc = Grib2DocumentReader::read_stream(&mut ww_reader).unwrap();
            let ww_grid = RegularGridConverter::create(&ww_doc, -1.0).unwrap();

            let ceiling_file = IconD2FileCeiling::get_file_url(&fc_step);
            let mut ceiling_reader = Self::get_file_reader(&ceiling_file);
            let ceiling_doc = Grib2DocumentReader::read_stream(&mut ceiling_reader).unwrap();
            let ceiling_grid = RegularGridConverter::create(&ceiling_doc, -1.0).unwrap();

            let weather_layer = DwdWeatherLayer::new(ww_grid, ceiling_grid).unwrap();

            let weather_bin = DwdWeatherMeteoBin::new(weather_layer);
            let data = weather_bin.create_bin_values();
            let filename = format!(
                "{}WW_D2.meteobin",
                Self::get_output_path(&fc_step, WEATHER_LAYER),
            );
            let mut file = BufWriter::new(File::create(&filename).expect("Unable to create file"));
            let _ = file.write_all(&data);
        });
    }


    fn create_wind_charts(forecast_run: &DwdForecastRun) {
        DwdForecastStep::get_step_range().into_par_iter().for_each(|step| {
            info!("creating wind charts, time step {}", step);

            let fc_step = DwdForecastStep::new_from_run(forecast_run, step);

            let wind_u_file = IconD2FileU10m::get_file_url(&fc_step);
            let mut wind_u_reader = Self::get_file_reader(&wind_u_file);
            let wind_u_doc = Grib2DocumentReader::read_stream(&mut wind_u_reader).unwrap();
            let wind_u_grid = RegularGridConverter::create(&wind_u_doc, -1.0).unwrap();

            let wind_v_file = IconD2FileV10m::get_file_url(&fc_step);
            let mut wind_v_reader = Self::get_file_reader(&wind_v_file);
            let wind_v_doc = Grib2DocumentReader::read_stream(&mut wind_v_reader).unwrap();
            let wind_v_grid = RegularGridConverter::create(&wind_v_doc, -1.0).unwrap();

            let wind_v_max_file = IconD2FileVmax10m::get_file_url(&fc_step);
            let mut wind_v_max_reader = Self::get_file_reader(&wind_v_max_file);
            let wind_v_max_doc = Grib2DocumentReader::read_stream(&mut wind_v_max_reader).unwrap();
            let wind_v_max_grid = RegularGridConverter::create(&wind_v_max_doc, -1.0).unwrap();

            let layer = DwdWindLayer::new(wind_u_grid, wind_v_grid, Some(wind_v_max_grid)).unwrap();

            // map tiles
            let _ = WindChartRenderer::render_map_tiles(
                &layer,
                (0, 7),
                |tile: &Drawable, zoom: u32, x: u32, y: u32| Self::save_tile_step(tile, zoom, x, y, WIND_LAYER, &fc_step)
            );

            // meteobin
            let wind_bin = DwdWindMeteobin::new(layer);
            let data = wind_bin.create_bin_values();
            let filename = format!(
                "{}WIND_D2.meteobin",
                Self::get_output_path(&fc_step, WIND_LAYER),
            );
            let mut file = BufWriter::new(File::create(&filename).expect("Unable to create wind meteobin file"));
            let _ = file.write_all(&data);
        });
    }


    fn get_file_reader(filename: &str) -> impl Read {
        info!("reading file {}", filename);
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
        layer: &str,
        fc_step: &DwdForecastStep
    ) {
        let path = format!(
            "{}{}/{}",
            Self::get_output_path(fc_step, layer),
            zoom,
            x
        );
        fs::create_dir_all(&path).unwrap();

        let filename = format!("{}/{}.png", &path, y);
        let _result = tile.safe_image(&filename);
    }


    fn get_output_path(
        fc_step: &DwdForecastStep,
        layer: &str
    ) -> String {
        return format!(
            "{}{}{}/{:03}/{}/",
            FORECAST_BASE_DIR,
            fc_step.run.start_date.format(DWD_DATE_FORMAT),
            fc_step.run.run_name.get_name(),
            fc_step.step,
            layer,
        );
    }
}
