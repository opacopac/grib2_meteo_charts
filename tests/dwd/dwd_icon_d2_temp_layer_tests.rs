use meteo_grib2_renderer::grib2::converter::file_to_grid_converter::FileToGridConverter;
use meteo_grib2_renderer::grib2::converter::regular_grid_converter::RegularGridConverter;
use meteo_grib2_renderer::meteo_layer::meteo_precip_layer::MeteoPrecipLayer;

pub const TEMP_TEST_FILE: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_single-level_2023080600_000_2d_t_2m.grib2";


#[test]
fn it_successfully_reads_a_temperature_test_file() {
    let doc = FileToGridConverter::read_single_doc_from_file_or_url(TEMP_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let _layer = MeteoPrecipLayer::new(grid);

    assert!(true);
}
