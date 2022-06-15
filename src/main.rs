#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use meteo_grib2_renderer::chart::cloud_chart_renderer::CloudChartRenderer;
use meteo_grib2_renderer::chart::cloud_precip_chart_renderer::CloudPrecipChartRenderer;
use meteo_grib2_renderer::chart::precip_chart_renderer::PrecipChartRenderer;
use meteo_grib2_renderer::chart::wind_chart_renderer::WindChartRenderer;
use meteo_grib2_renderer::chart::ww_chart_renderer::WwChartRenderer;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::imaging::drawable::Drawable;
use meteo_grib2_renderer::meteo_dwd::dwd_cloud_layer::DwdCloudLayer;
use meteo_grib2_renderer::meteo_dwd::dwd_cloud_precip_layer::DwdCloudPrecipLayer;
use meteo_grib2_renderer::meteo_dwd::dwd_precip_layer::DwdPrecipLayer;
use meteo_grib2_renderer::meteo_dwd::dwd_wind_layer::DwdWindLayer;
use meteo_grib2_renderer::meteo_dwd::dwd_ww_layer::DwdWwLayer;
use meteo_grib2_renderer::meteo_dwd::regular_grid_converter::RegularGridConverter;
use meteo_grib2_renderer::meteo_dwd::unstructured_grid_converter::{CLAT_VAR_NAME, CLON_VAR_NAME, UnstructuredGridConverter};
use meteo_grib2_renderer::metobin::wind_metobin::WindMeteobin;
use meteo_grib2_renderer::metobin::ww_metobin::WwMeteoBin;
use meteo_grib2_renderer::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

const CLCT_TEST_FILE_D2: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_clct_mod.grib2";
const CLCT_TEST_FILE_EU: &str = "icon-eu_europe_regular-lat-lon_single-level_2022042700_047_CLCT_MOD.grib2";
const CLCT_TEST_FILE_GLOBAL: &str = "icon_global_icosahedral_single-level_2022060412_020_CLCT_MOD.grib2";
const PRECIP_TEST_FILE: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042700_048_2d_tot_prec.grib2";
const WW_TEST_FILE_D2: &str = "icon-d2_germany_regular-lat-lon_single-level_2022053118_010_2d_ww.grib2";
const CP_PRECIP0_TEST_FILE_D2: &str = "icon-d2_germany_regular-lat-lon_single-level_2022053118_009_2d_tot_prec.grib2";
const CP_PRECIP1_TEST_FILE_D2: &str = "icon-d2_germany_regular-lat-lon_single-level_2022053118_010_2d_tot_prec.grib2";
const CP_PRECIP0_TEST_FILE_GLOBAL: &str = "icon_global_icosahedral_single-level_2022060412_020_TOT_PREC.grib2";
const CP_PRECIP1_TEST_FILE_GLOBAL: &str = "icon_global_icosahedral_single-level_2022060412_021_TOT_PREC.grib2";
const CP_CLCT_TEST_FILE_D2: &str = "icon-d2_germany_regular-lat-lon_single-level_2022053118_010_2d_clct_mod.grib2";
const WIND_U_TEST_FILE_D2: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_u_10m.grib2";
const WIND_V_TEST_FILE_D2: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_v_10m.grib2";
//const WIND_U_TEST_FILE_EU: &str = "icon-eu_europe_regular-lat-lon_single-level_2022051015_000_U_10M.grib2";
//const WIND_V_TEST_FILE_EU: &str = "icon-eu_europe_regular-lat-lon_single-level_2022051015_000_V_10M.grib2";
const NETCDF_ICON_GRID_TEST_FILE: &str = "icon_grid_0026_R03B07_G.nc";

fn main() {
    //create_icon_d2_precip_img();
    //create_icon_d2_clct_img();
    //create_icon_eu_clct_img();
    //create_icon_global_clct_img();
    //create_icon_global_cloud_precip_img();
    //create_icon_d2_wind_img();
    //create_icon_d2_wind_map_tile();
    //create_icon_d2_cloud_precip_img();
    //create_icon_d2_ww_img();

    //create_icon_d2_clct_precip_map_tiles();
    //create_icon_d2_map_tiles();
    //create_icon_global_map_tiles();

    create_icon_d2_clct_precip_map_tile_series();
    create_icon_d2_wind_map_tile_series();
    create_icon_d2_wind_binary_series();
    create_icon_d2_ww_binary_series();

    //create_icon_global_clct_precip_map_tile_series();
    //perf_icon_global();

    //create_icon_d2_wind_binary();
    //create_icon_d2_ww_binary();
}


fn create_icon_d2_clct_precip_map_tiles() {
    let clct_doc = Grib2DocumentReader::read_file(CP_CLCT_TEST_FILE_D2).unwrap();
    let clct_grid = RegularGridConverter::create(&clct_doc, -1.0).unwrap();
    let precip_doc0 = Grib2DocumentReader::read_file(CP_PRECIP0_TEST_FILE_D2).unwrap();
    let precip_grid0 = RegularGridConverter::create(&precip_doc0, -1.0).unwrap();
    let precip_doc1 = Grib2DocumentReader::read_file(CP_PRECIP1_TEST_FILE_D2).unwrap();
    let precip_grid1 = RegularGridConverter::create(&precip_doc1, -1.0).unwrap();

    let layer = DwdCloudPrecipLayer::new(clct_grid, precip_grid0, precip_grid1).unwrap();
    let _ = CloudPrecipChartRenderer::render_map_tiles(
        &layer,
        (0, 7),
        |tile: &Drawable, zoom: u32, x: u32, y: u32| save_tile(tile, zoom, x, y)
    );
}


fn create_icon_d2_clct_precip_map_tile_series() {
    let base_dir: &str = "./set01/";
    let clct_file_prefix: &str = "icon-d2_germany_regular-lat-lon_single-level_2022061515_";
    let clct_file_suffix: &str = "_2d_clct_mod.grib2";
    let precip_file_prefix: &str = "icon-d2_germany_regular-lat-lon_single-level_2022061515_";
    let precip_file_suffix: &str = "_2d_tot_prec.grib2";

    for i in 2..=8 {
        println!("time step {}", i);

        let nr0 = format!("{:03}", i);
        let nr_m1 = format!("{:03}", i - 1);
        let clct_file = format!("{}{}{}{}", base_dir, clct_file_prefix, &nr0, clct_file_suffix);
        let precip_file0 = format!("{}{}{}{}", base_dir, precip_file_prefix, &nr_m1, precip_file_suffix);
        let precip_file1 = format!("{}{}{}{}", base_dir, precip_file_prefix, &nr0, precip_file_suffix);
        let clct_doc = Grib2DocumentReader::read_file(&clct_file).unwrap();
        let clct_grid = RegularGridConverter::create(&clct_doc, -1.0).unwrap();
        let precip_doc0 = Grib2DocumentReader::read_file(&precip_file0).unwrap();
        let precip_grid0 = RegularGridConverter::create(&precip_doc0, -1.0).unwrap();
        let precip_doc1 = Grib2DocumentReader::read_file(&precip_file1).unwrap();
        let precip_grid1 = RegularGridConverter::create(&precip_doc1, -1.0).unwrap();

        let layer = DwdCloudPrecipLayer::new(clct_grid, precip_grid0, precip_grid1).unwrap();
        let save_dir = format!("{}clct_precip/{}/", &base_dir, &nr0);
        let _ = CloudPrecipChartRenderer::render_map_tiles(
            &layer,
            (0, 7),
            |tile: &Drawable, zoom: u32, x: u32, y: u32| save_tile_step(tile, zoom, x, y, &save_dir)
        );
    }
}


fn create_icon_d2_wind_map_tile_series() {
    let base_dir: &str = "./set01/";
    let wind_u_file_prefix: &str = "icon-d2_germany_regular-lat-lon_single-level_2022061515_";
    let wind_u_file_suffix: &str = "_2d_u_10m.grib2";
    let wind_v_file_prefix: &str = "icon-d2_germany_regular-lat-lon_single-level_2022061515_";
    let wind_v_file_suffix: &str = "_2d_v_10m.grib2";

    for i in 0..=8 {
        println!("time step {}", i);

        let nr0 = format!("{:03}", i);
        let wind_u_file = format!("{}{}{}{}", base_dir, wind_u_file_prefix, &nr0, wind_u_file_suffix);
        let wind_v_file = format!("{}{}{}{}", base_dir, wind_v_file_prefix, &nr0, wind_v_file_suffix);
        let wind_u_doc = Grib2DocumentReader::read_file(&wind_u_file).unwrap();
        let wind_u_grid = RegularGridConverter::create(&wind_u_doc, -1.0).unwrap();
        let wind_v_doc = Grib2DocumentReader::read_file(&wind_v_file).unwrap();
        let wind_v_grid0 = RegularGridConverter::create(&wind_v_doc, -1.0).unwrap();

        let layer = DwdWindLayer::new(wind_u_grid, wind_v_grid0).unwrap();
        let save_dir = format!("{}wind/{}/", &base_dir, &nr0);
        let _ = WindChartRenderer::render_map_tiles(
            &layer,
            (0, 7),
            |tile: &Drawable, zoom: u32, x: u32, y: u32| save_tile_step(tile, zoom, x, y, &save_dir)
        );
    }
}



fn create_icon_d2_wind_binary_series() {
    let base_dir: &str = "./set01/";
    let wind_u_file_prefix: &str = "icon-d2_germany_regular-lat-lon_single-level_2022061515_";
    let wind_u_file_suffix: &str = "_2d_u_10m.grib2";
    let wind_v_file_prefix: &str = "icon-d2_germany_regular-lat-lon_single-level_2022061515_";
    let wind_v_file_suffix: &str = "_2d_v_10m.grib2";

    for i in 0..=8 {
        println!("time step {}", i);

        let nr0 = format!("{:03}", i);
        let wind_u_file = format!("{}{}{}{}", base_dir, wind_u_file_prefix, &nr0, wind_u_file_suffix);
        let wind_v_file = format!("{}{}{}{}", base_dir, wind_v_file_prefix, &nr0, wind_v_file_suffix);
        let doc_u = Grib2DocumentReader::read_file(&wind_u_file).unwrap();
        let doc_v = Grib2DocumentReader::read_file(&wind_v_file).unwrap();
        let grid_u = RegularGridConverter::create(&doc_u, -1.0).unwrap();
        let grid_v = RegularGridConverter::create(&doc_v, -1.0).unwrap();
        let layer = DwdWindLayer::new(grid_u, grid_v).unwrap();
        let wind_bin = WindMeteobin::new(layer);
        let data = wind_bin.create_bin_values();

        let filename = format!("{}wind/{}/WIND_D2.meteobin", &base_dir, &nr0);
        let mut file = BufWriter::new(File::create(&filename).expect("Unable to create wind meteobin file"));
        let _ = file.write_all(&data);
    }
}


fn create_icon_d2_ww_binary_series() {
    let base_dir: &str = "./set01/";
    let ww_file_prefix: &str = "icon-d2_germany_regular-lat-lon_single-level_2022061515_";
    let ww_file_suffix: &str = "_2d_ww.grib2";

    for i in 2..=8 {
        println!("time step {}", i);

        let nr0 = format!("{:03}", i);
        let ww_file = format!("{}{}{}{}", base_dir, ww_file_prefix, &nr0, ww_file_suffix);
        let doc = Grib2DocumentReader::read_file(&ww_file).unwrap();
        let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
        let layer = DwdWwLayer::new(grid);
        let ww_bin = WwMeteoBin::new(layer);
        let data = ww_bin.create_bin_values();

        let filename = format!("{}clct_precip/{}/WW_D2.meteobin", &base_dir, &nr0);
        let mut file = BufWriter::new(File::create(&filename).expect("Unable to create file"));
        let _ = file.write_all(&data);
    }
}


fn create_icon_global_clct_precip_map_tile_series() {
    let clct_file_prefix: &str = "icon_global_icosahedral_single-level_2022060412_";
    let clct_file_suffix: &str = "_CLCT_MOD.grib2";
    let precip_file_prefix: &str = "icon_global_icosahedral_single-level_2022060412_";
    let precip_file_suffix: &str = "_TOT_PREC.grib2";
    let netcdf_doc = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE, vec![CLAT_VAR_NAME, CLON_VAR_NAME]).unwrap();

    for i in 20..=27 {
        println!("time step {}", i);

        let nr0 = format!("{:03}", i);
        let nr1 = format!("{:03}", i + 1);
        let clct_file = format!("{}{}{}", clct_file_prefix, &nr0, clct_file_suffix);
        let precip_file0 = format!("{}{}{}", precip_file_prefix, &nr0, precip_file_suffix);
        let precip_file1 = format!("{}{}{}", precip_file_prefix, &nr1, precip_file_suffix);
        let clct_doc = Grib2DocumentReader::read_file(&clct_file).unwrap();
        let clct_grid = UnstructuredGridConverter::create(&clct_doc, -1.0, &netcdf_doc).unwrap();
        let precip_doc0 = Grib2DocumentReader::read_file(&precip_file0).unwrap();
        let precip_grid0 = UnstructuredGridConverter::create(&precip_doc0, -1.0, &netcdf_doc).unwrap();
        let precip_doc1 = Grib2DocumentReader::read_file(&precip_file1).unwrap();
        let precip_grid1 = UnstructuredGridConverter::create(&precip_doc1, -1.0, &netcdf_doc).unwrap();

        let layer = DwdCloudPrecipLayer::new(clct_grid, precip_grid0, precip_grid1).unwrap();
        let dir = &format!("./{}/", &nr0);
        let _ = CloudPrecipChartRenderer::render_map_tiles(
            &layer,
            (0, 5),
            |tile: &Drawable, zoom: u32, x: u32, y: u32| save_tile_step(tile, zoom, x, y, &nr0)
        );
    }
}


fn create_icon_d2_clct_img() {
    let start = Instant::now();

    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_D2).unwrap();
    let elapsed = start.elapsed();
    println!("read doc {}", elapsed.as_millis());

    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = DwdCloudLayer::new(grid);
    let elapsed = start.elapsed();
    println!("create ccl {}", elapsed.as_millis());

    let img = CloudChartRenderer::render_full_chart(&layer).unwrap();
    let elapsed = start.elapsed();
    println!("create img {}", elapsed.as_millis());

    img.safe_image("CLCT.png").unwrap();
    let elapsed = start.elapsed();
    println!("save img {}", elapsed.as_millis());
}


fn create_icon_eu_clct_img() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_EU).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = DwdCloudLayer::new(grid);
    let img = CloudChartRenderer::render_full_chart(&layer).unwrap();
    img.safe_image("CLCT_EU2.png").unwrap();
}


fn create_icon_global_clct_img() {
    let grib_doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_GLOBAL).unwrap();
    let netcdf_doc = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE, vec![CLAT_VAR_NAME, CLON_VAR_NAME]).unwrap(); // TODO
    let grid = UnstructuredGridConverter::create(&grib_doc, -1.0, &netcdf_doc).unwrap();
    let layer = DwdCloudLayer::new(grid);
    let img = CloudChartRenderer::render_full_chart(&layer).unwrap();

    img.safe_image("CLCT_GLOBAL2.png").unwrap();
}


fn create_icon_global_cloud_precip_img() {
    let netcdf_doc = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE, vec![CLAT_VAR_NAME, CLON_VAR_NAME]).unwrap(); // TODO
    let doc_cl = Grib2DocumentReader::read_file(CLCT_TEST_FILE_GLOBAL).unwrap();
    let doc_p0 = Grib2DocumentReader::read_file(CP_PRECIP0_TEST_FILE_GLOBAL).unwrap();
    let doc_p1 = Grib2DocumentReader::read_file(CP_PRECIP1_TEST_FILE_GLOBAL).unwrap();
    let grid_cl = UnstructuredGridConverter::create(&doc_cl, -1.0, &netcdf_doc).unwrap();
    let grid_p0 = UnstructuredGridConverter::create(&doc_p0, -1.0, &netcdf_doc).unwrap();
    let grid_p1 = UnstructuredGridConverter::create(&doc_p1, -1.0, &netcdf_doc).unwrap();
    let layer = DwdCloudPrecipLayer::new(grid_cl, grid_p0, grid_p1).unwrap();
    let img = CloudPrecipChartRenderer::render_full_chart(&layer).unwrap();
    img.safe_image("CLOUD_PRECIP_GLOBAL.png").unwrap();
}


fn create_icon_d2_precip_img() {
    let doc = Grib2DocumentReader::read_file(PRECIP_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = DwdPrecipLayer::new(grid);
    let img = PrecipChartRenderer::render_full_chart(&layer).unwrap();
    img.safe_image("PRECIP2.png").unwrap();
}


fn create_icon_d2_wind_img() {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE_D2).unwrap();
    let doc_v = Grib2DocumentReader::read_file(WIND_V_TEST_FILE_D2).unwrap();
    let grid_u = RegularGridConverter::create(&doc_u, -1.0).unwrap();
    let grid_v = RegularGridConverter::create(&doc_v, -1.0).unwrap();
    let layer = DwdWindLayer::new(grid_u, grid_v).unwrap();
    let img = WindChartRenderer::render_full_chart(&layer).unwrap();
    img.safe_image("WIND2.png").unwrap();
}


fn create_icon_d2_cloud_precip_img() {
    let doc_cl = Grib2DocumentReader::read_file(CP_CLCT_TEST_FILE_D2).unwrap();
    let doc_p0 = Grib2DocumentReader::read_file(CP_PRECIP0_TEST_FILE_D2).unwrap();
    let doc_p1 = Grib2DocumentReader::read_file(CP_PRECIP1_TEST_FILE_D2).unwrap();
    let grid_cl = RegularGridConverter::create(&doc_cl, -1.0).unwrap();
    let grid_p0 = RegularGridConverter::create(&doc_p0, -1.0).unwrap();
    let grid_p1 = RegularGridConverter::create(&doc_p1, -1.0).unwrap();
    let layer = DwdCloudPrecipLayer::new(grid_cl, grid_p0, grid_p1).unwrap();
    let img = CloudPrecipChartRenderer::render_full_chart(&layer).unwrap();
    img.safe_image("CLOUD_PRECIP.png").unwrap();
}


fn create_icon_d2_ww_img() {
    let doc = Grib2DocumentReader::read_file(WW_TEST_FILE_D2).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = DwdWwLayer::new(grid);
    let img = WwChartRenderer::render_full_chart(&layer).unwrap();
    img.safe_image("WW.png").unwrap();
}


fn create_icon_d2_map_tiles() {
    let start = Instant::now();

    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_D2).unwrap();
    let elapsed = start.elapsed();
    println!("read doc {}", elapsed.as_millis());

    //let ccl = DwdIconD2TotalCloudCoverLayer::from_grib2(doc).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let ccl = DwdCloudLayer::new(grid);
    let elapsed = start.elapsed();
    println!("create ccl {}", elapsed.as_millis());

    let _ = CloudChartRenderer::render_map_tiles(
        &ccl,
        (0, 7),
        |tile: &Drawable, zoom: u32, x: u32, y: u32| save_tile(tile, zoom, x, y)
    );
    let elapsed = start.elapsed();
    println!("create img {}", elapsed.as_millis());

    let elapsed = start.elapsed();
    println!("save img {}", elapsed.as_millis());
}


fn create_icon_global_map_tiles() {
    let grib_doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_GLOBAL).unwrap();
    let netcdf_doc = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE, vec![CLAT_VAR_NAME, CLON_VAR_NAME]).unwrap(); // TODO
    let grid = UnstructuredGridConverter::create(&grib_doc, -1.0, &netcdf_doc).unwrap();
    let ccl = DwdCloudLayer::new(grid);
    let _ = CloudChartRenderer::render_map_tiles(
        &ccl,
        (0, 5),
        |tile: &Drawable, zoom: u32, x: u32, y: u32| save_tile(tile, zoom, x, y)
    );
}


fn create_icon_d2_wind_map_tile() {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE_D2).unwrap();
    let doc_v = Grib2DocumentReader::read_file(WIND_V_TEST_FILE_D2).unwrap();
    let grid_u = RegularGridConverter::create(&doc_u, -1.0).unwrap();
    let grid_v = RegularGridConverter::create(&doc_v, -1.0).unwrap();
    let layer = DwdWindLayer::new(grid_u, grid_v).unwrap();
    let _ = WindChartRenderer::render_map_tiles(
        &layer,
        (0, 7),
        |tile: &Drawable, zoom: u32, x: u32, y: u32| save_tile(tile, zoom, x, y)
    );
}


fn create_icon_d2_wind_binary() {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE_D2).unwrap();
    let doc_v = Grib2DocumentReader::read_file(WIND_V_TEST_FILE_D2).unwrap();
    let grid_u = RegularGridConverter::create(&doc_u, -1.0).unwrap();
    let grid_v = RegularGridConverter::create(&doc_v, -1.0).unwrap();
    let layer = DwdWindLayer::new(grid_u, grid_v).unwrap();
    let wind_bin = WindMeteobin::new(layer);
    let data = wind_bin.create_bin_values();
    let mut file = BufWriter::new(File::create("WIND_D2.meteobin").expect("Unable to create file"));
    let _ = file.write_all(&data);
}


fn create_icon_d2_ww_binary() {
    let doc = Grib2DocumentReader::read_file(WW_TEST_FILE_D2).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = DwdWwLayer::new(grid);
    let ww_bin = WwMeteoBin::new(layer);
    let data = ww_bin.create_bin_values();
    let mut file = BufWriter::new(File::create("WW_D2.meteobin").expect("Unable to create file"));
    let _ = file.write_all(&data);
}


fn save_tile(
    tile: &Drawable,
    zoom: u32,
    x: u32,
    y: u32
) {
    let base_path = "./007/";
    let path = format!("{}/{}/{}", base_path, zoom, x);
    fs::create_dir_all(&path).unwrap();

    let filename = format!("{}/{}.png", &path, y);
    let _result = tile.safe_image(&filename);
}


fn save_tile_step(
    tile: &Drawable,
    zoom: u32,
    x: u32,
    y: u32,
    step: &str
) {
    let base_path = format!("./{}/", step);
    let path = format!("{}/{}/{}", base_path, zoom, x);
    fs::create_dir_all(&path).unwrap();

    let filename = format!("{}/{}.png", &path, y);
    let _result = tile.safe_image(&filename);
}
