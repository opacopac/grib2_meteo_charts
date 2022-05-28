#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::time::Instant;

use meteo_grib2_renderer::chart::cloud_chart_renderer2::CloudChartRenderer2;
use meteo_grib2_renderer::chart::precip_chart_renderer2::PrecipChartRenderer2;
use meteo_grib2_renderer::chart::wind_chart_renderer2::WindChartRenderer2;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::imaging::drawable::Drawable;
use meteo_grib2_renderer::meteo_dwd::dwd_cloud_layer2::DwdCloudLayer2;
use meteo_grib2_renderer::meteo_dwd::dwd_precip_layer2::DwdPrecipLayer2;
use meteo_grib2_renderer::meteo_dwd::dwd_wind_layer2::DwdWindLayer2;
use meteo_grib2_renderer::meteo_dwd::regular_grid_converter::RegularGridConverter;
use meteo_grib2_renderer::meteo_dwd::unstructured_grid_converter::{CLAT_VAR_NAME, CLON_VAR_NAME, UnstructuredGridConverter};
use meteo_grib2_renderer::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

const CLCT_TEST_FILE_D2: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_clct_mod.grib2";
const CLCT_TEST_FILE_EU: &str = "icon-eu_europe_regular-lat-lon_single-level_2022042700_047_CLCT_MOD.grib2";
const CLCT_TEST_FILE_GLOBAL: &str = "icon_global_icosahedral_single-level_2022051300_000_CLCT_MOD.grib2";
const PRECIP_TEST_FILE: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042700_048_2d_tot_prec.grib2";
//const WIND_U_TEST_FILE: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_u_10m.grib2";
//const WIND_V_TEST_FILE: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_v_10m.grib2";
const WIND_U_TEST_FILE: &str = "icon-eu_europe_regular-lat-lon_single-level_2022051015_000_U_10M.grib2";
const WIND_V_TEST_FILE: &str = "icon-eu_europe_regular-lat-lon_single-level_2022051015_000_V_10M.grib2";
const NETCDF_ICON_GRID_TEST_FILE: &str = "icon_grid_0026_R03B07_G.nc";

fn main() {
    //create_icon_d2_precip_img();
    //create_icon_d2_clct_img();
    //create_icon_eu_clct_img();
    //create_icon_global_clct_img();
    //create_icon_d2_wind_img();
    create_icon_d2_wind_map_tile();

    //create_icon_d2_map_tiles();
    //create_icon_global_map_tiles();
    //create_icon_d2_map_tile_series();
    //perf_icon_global();
}

/*fn perf_icon_global() {
    let grib2_doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_D2).unwrap();
    let grid = DwdIconGlobalGridReader::create(NETCDF_ICON_GRID_TEST_FILE).unwrap();
    let layer = DwdIconGlobalTotalCloudCoverLayer::create(grib2_doc, grid).unwrap();

    let mut rng = rand::thread_rng();
    let start = Instant::now();
    for _ in 0..1000000 {
        let pos = &LatLon::new(rng.gen::<f32>() * 180.0 - 90.0, rng.gen::<f32>() * 360.0 - 180.0);
        let value= layer.grid.find_closest_point_value(pos);
    }
    println!("reading from grid: {}", start.elapsed().as_millis());
}*/


fn create_icon_d2_map_tile_series() {
    let file_prefix: &str = "icon-d2_germany_regular-lat-lon_single-level_2022042415_";
    let file_suffix: &str = "_2d_clct_mod.grib2";

    for i in 0..=7 {
        let nr = format!("{:03}", i);
        let file = format!("{}{}{}", file_prefix, &nr, file_suffix);
        //println!("{}", file);

        let doc = Grib2DocumentReader::read_file(&file).unwrap();
        let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
        let ccl = DwdCloudLayer2::new(grid);
        let dir = &format!("./{}/", &nr);
        let _ = CloudChartRenderer2::render_map_tiles(
            &ccl,
            (0, 7),
            |tile: &Drawable, zoom: u32, x: u32, y: u32| save_tile(tile, zoom, x, y)
        );
    }
}


fn create_icon_d2_clct_img() {
    let start = Instant::now();

    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_D2).unwrap();
    let elapsed = start.elapsed();
    println!("read doc {}", elapsed.as_millis());

    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = DwdCloudLayer2::new(grid);
    let elapsed = start.elapsed();
    println!("create ccl {}", elapsed.as_millis());

    let img = CloudChartRenderer2::render_full_chart(&layer).unwrap();
    let elapsed = start.elapsed();
    println!("create img {}", elapsed.as_millis());

    img.safe_image("CLCT.png").unwrap();
    let elapsed = start.elapsed();
    println!("save img {}", elapsed.as_millis());
}


fn create_icon_eu_clct_img() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_EU).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = DwdCloudLayer2::new(grid);
    let img = CloudChartRenderer2::render_full_chart(&layer).unwrap();
    img.safe_image("CLCT_EU2.png").unwrap();
}


fn create_icon_global_clct_img() {
    let grib_doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_GLOBAL).unwrap();
    let netcdf_doc = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE, vec![CLAT_VAR_NAME, CLON_VAR_NAME]).unwrap(); // TODO
    let grid = UnstructuredGridConverter::create(&grib_doc, -1.0, &netcdf_doc).unwrap();
    let layer = DwdCloudLayer2::new(grid);
    let img = CloudChartRenderer2::render_full_chart(&layer).unwrap();

    img.safe_image("CLCT_GLOBAL2.png").unwrap();
}


fn create_icon_d2_precip_img() {
    let doc = Grib2DocumentReader::read_file(PRECIP_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = DwdPrecipLayer2::new(grid);
    let img = PrecipChartRenderer2::render_full_chart(&layer).unwrap();
    img.safe_image("PRECIP2.png").unwrap();
}


fn create_icon_d2_wind_img() {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE).unwrap();
    let doc_v = Grib2DocumentReader::read_file(WIND_V_TEST_FILE).unwrap();
    let grid_u = RegularGridConverter::create(&doc_u, -1.0).unwrap();
    let grid_v = RegularGridConverter::create(&doc_v, -1.0).unwrap();
    let layer = DwdWindLayer2::new(grid_u, grid_v).unwrap();
    let img = WindChartRenderer2::render_full_chart(&layer).unwrap();
    img.safe_image("WIND2.png").unwrap();
}


fn create_icon_d2_map_tiles() {
    let start = Instant::now();

    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE_D2).unwrap();
    let elapsed = start.elapsed();
    println!("read doc {}", elapsed.as_millis());

    //let ccl = DwdIconD2TotalCloudCoverLayer::from_grib2(doc).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let ccl = DwdCloudLayer2::new(grid);
    let elapsed = start.elapsed();
    println!("create ccl {}", elapsed.as_millis());

    let _ = CloudChartRenderer2::render_map_tiles(
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
    let ccl = DwdCloudLayer2::new(grid);
    let _ = CloudChartRenderer2::render_map_tiles(
        &ccl,
        (0, 2),
        |tile: &Drawable, zoom: u32, x: u32, y: u32| save_tile(tile, zoom, x, y)
    );
}


fn create_icon_d2_wind_map_tile() {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE).unwrap();
    let doc_v = Grib2DocumentReader::read_file(WIND_V_TEST_FILE).unwrap();
    let grid_u = RegularGridConverter::create(&doc_u, -1.0).unwrap();
    let grid_v = RegularGridConverter::create(&doc_v, -1.0).unwrap();
    let layer = DwdWindLayer2::new(grid_u, grid_v).unwrap();
    let _ = WindChartRenderer2::render_map_tiles(
        &layer,
        (0, 5),
        |tile: &Drawable, zoom: u32, x: u32, y: u32| save_tile(tile, zoom, x, y)
    );
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
