use crate::meteo_swiss::file_reader::icon_ch1_file_service::IconCh1FileService;


pub trait MeteoSwissDiContainer: Send + Sync {
    fn get_icon_ch1_file_service(&mut self) -> &IconCh1FileService;
}


pub struct ProdMeteoSwissDiContainer {
    icon_d2_file_service: Option<IconCh1FileService>,
}


impl ProdMeteoSwissDiContainer {
    pub fn new() -> Self {
        Self {
            icon_d2_file_service: None,
        }
    }
}


impl MeteoSwissDiContainer for ProdMeteoSwissDiContainer {
    fn get_icon_ch1_file_service(&mut self) -> &IconCh1FileService {
        if self.icon_d2_file_service.is_none() {
            self.icon_d2_file_service = Some(IconCh1FileService::new());
        }

        self.icon_d2_file_service.as_ref().unwrap()
    }
}
