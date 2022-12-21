use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grib2::section3::grid_definition_template::GridDefinitionTemplate::LatitudeLongitude;
use meteo_grib2_renderer::dwd_layer::dwd_cloud_layer::DwdCloudLayer;
use meteo_grib2_renderer::grid::regular_grid_converter::RegularGridConverter;

pub const CLCT_TEST_FILE: &str = "./tests/resources/icon-eu_europe_regular-lat-lon_single-level_2022042700_047_CLCT_MOD.grib2";


#[test]
fn it_successfully_reads_an_icon_eu_clct_test_file() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();

    match doc.section3.grid_definition_template {
        LatitudeLongitude(grid) => {
            println!("{} {}", grid.first_grid_point_lat, grid.first_grid_point_lon);
            println!("{} {}", grid.last_grid_point_lat, grid.last_grid_point_lon);
            println!("{} {}", grid.number_of_points_along_parallel, grid.number_of_points_along_meridian);
            println!("{} {}", grid.i_direction_increment, grid.j_direction_increment);
        }
        _ => {}
    }



    let _layer = DwdCloudLayer::new(grid);

    assert!(true);
}
