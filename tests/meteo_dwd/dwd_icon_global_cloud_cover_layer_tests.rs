use meteo_grib2_renderer::geo::lat_lon::LatLon;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grib2::section3::grid_definition_template::GridDefinitionTemplate;
use meteo_grib2_renderer::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
use meteo_grib2_renderer::meteo_dwd::dwd_icon_global_grid_reader::DwdIconGlobalGridReader;
use meteo_grib2_renderer::meteo_dwd::dwd_icon_global_tot_cloud_cover_layer::DwdIconGlobalTotalCloudCoverLayer;

pub const CLCT_TEST_FILE: &str = "./tests/data/icon_global_icosahedral_single-level_2022051300_000_CLCT_MOD.grib2";
pub const NETCDF_ICON_GRID_TEST_FILE: &str = "./tests/data/icon_grid_0009_R02B03_R.nc";


#[test]
fn it_successfully_reads_an_icon_global_clct_test_file() {
    let grib2_doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();

    match &grib2_doc.section3.grid_definition_template {
        GridDefinitionTemplate::UnstructuredGrid(tpl) => {
            assert_eq!(26, tpl.number_of_grid);
            assert_eq!(1, tpl.number_of_grid_in_ref);
        },
        _ => panic!("invalid grid definition template: {:?}", grib2_doc.section3.grid_definition_template)
    }

    let grid = DwdIconGlobalGridReader::create(NETCDF_ICON_GRID_TEST_FILE).unwrap();

    let result = DwdIconGlobalTotalCloudCoverLayer::create(grib2_doc, grid);
    assert!(result.is_ok());

    let layer = result.unwrap();
    assert_eq!(MeteoParameterCategory::Cloud, layer.parameter_category);
    assert_eq!(199, layer.parameter_number);

    for i in 0..layer.grid.get_point_count() {
        let point = layer.grid.get_point_by_idx(i);
        println!("point {}: {:?}", i, point);
    }

    let idx = layer.grid.get_idx_by_lat_lon(&LatLon::new(47.0, 7.0));
    let pt = layer.grid.get_point_by_idx(idx);
    println!("CH point: {:?}", pt);

    let idx = layer.grid.get_idx_by_lat_lon(&LatLon::new(48.0, 8.0));
    let pt = layer.grid.get_point_by_idx(idx);
    println!("CH point: {:?}", pt);

    let idx = layer.grid.get_idx_by_lat_lon(&LatLon::new(49.0, 9.0));
    let pt = layer.grid.get_point_by_idx(idx);
    println!("CH point: {:?}", pt);
}
