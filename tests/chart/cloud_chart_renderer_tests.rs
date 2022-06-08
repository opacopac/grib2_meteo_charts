use meteo_grib2_renderer::chart::cloud_chart_renderer::CloudChartRenderer;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::meteo_dwd::dwd_cloud_layer::DwdCloudLayer;
use meteo_grib2_renderer::meteo_dwd::regular_grid_converter::RegularGridConverter;

pub const CLCT_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";


pub fn read_test_cloud_layer() -> DwdCloudLayer {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = DwdCloudLayer::new(grid);

    return layer;
}

#[test]
fn it_creates_an_image_from_a_cloud_layer_with_the_correct_dimensions() {
    let layer = read_test_cloud_layer();

    let result = CloudChartRenderer::render_full_chart(&layer);
    assert!(!result.is_err());

    let result = result.unwrap();
    assert_eq!(1215, result.width());
    assert_eq!(746, result.height());
}
