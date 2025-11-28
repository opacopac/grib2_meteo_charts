use crate::dwd::dwd_file_reader::icon_d2_file_service::IconD2FileService;


pub trait DwdDiContainer: Send + Sync {
    fn get_icon_d2_file_service(&mut self) -> &IconD2FileService;
}
