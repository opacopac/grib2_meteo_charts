use crate::dwd::dwd_file_reader::icon_d2_file_service::IconD2FileService;


pub struct DwdDiContainer {
    icon_d2_file_service: IconD2FileService,
}


impl DwdDiContainer {
    pub fn create_productive() -> Self {
        Self {
            icon_d2_file_service: IconD2FileService::new(),
        }
    }


    pub fn get_icon_d2_file_service(&self) -> &IconD2FileService {
        &self.icon_d2_file_service
    }
}
