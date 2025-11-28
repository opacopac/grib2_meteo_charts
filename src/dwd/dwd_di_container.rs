use crate::dwd::dwd_file_reader::icon_d2_file_service::IconD2FileService;


pub trait DwdDiContainer: Send + Sync {
    fn get_icon_d2_file_service(&mut self) -> &IconD2FileService;
}


pub struct ProdDwdDiContainer {
    icon_d2_file_service: Option<IconD2FileService>,
}


impl ProdDwdDiContainer {
    pub fn new() -> Self {
        Self {
            icon_d2_file_service: None,
        }
    }
}


impl DwdDiContainer for ProdDwdDiContainer {
    fn get_icon_d2_file_service(&mut self) -> &IconD2FileService {
        if self.icon_d2_file_service.is_none() {
            self.icon_d2_file_service = Some(IconD2FileService::new());
        }

        self.icon_d2_file_service.as_ref().unwrap()
    }
}
