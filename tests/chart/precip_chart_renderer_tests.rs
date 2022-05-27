use meteo_grib2_renderer::chart::precip_chart_renderer2::PrecipChartRenderer2;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::meteo_dwd::dwd_precip_layer2::DwdPrecipLayer2;
use meteo_grib2_renderer::meteo_dwd::regular_grid_converter::RegularGridConverter;

pub const PRECIP_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022042500_001_2d_tot_prec.grib2";


pub fn read_test_precip_layer() -> DwdPrecipLayer2 {
    let doc = Grib2DocumentReader::read_file(PRECIP_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = DwdPrecipLayer2::new(grid);

    return layer;
}

#[test]
fn it_creates_an_image_from_a_precip_layer_with_the_correct_dimensions() {
    let layer = read_test_precip_layer();

    let result = PrecipChartRenderer2::render_full_chart(&layer);
    assert!(!result.is_err());

    let result = result.unwrap();
    assert_eq!(1215, result.width());
    assert_eq!(746, result.height());
}
