use crate::meteo_swiss::file_reader::icon_ch1_file_service::IconCh1FileService;
use crate::meteo_swiss::meteo_swiss_di_container::MeteoSwissDiContainer;


pub struct TestMeteoSwissDiContainer {}


impl TestMeteoSwissDiContainer {
    pub fn new() -> Self {
        Self {}
    }
}


impl MeteoSwissDiContainer for TestMeteoSwissDiContainer {
    fn get_icon_ch1_file_service(&mut self) -> &IconCh1FileService {
        todo!("not implemented yet")
    }
}
