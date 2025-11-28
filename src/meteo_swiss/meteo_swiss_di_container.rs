use crate::meteo_swiss::file_reader::icon_ch1_file_service::IconCh1FileService;


pub struct MeteoSwissDiContainer {
    icon_d2_file_service: IconCh1FileService,
}


impl MeteoSwissDiContainer {
    pub fn create_productive() -> Self {
        Self {
            icon_d2_file_service: IconCh1FileService::new(),
        }
    }


    pub fn get_icon_ch1_file_service(&self) -> &IconCh1FileService {
        &self.icon_d2_file_service
    }
}
