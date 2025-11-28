use crate::meteo_swiss::file_reader::icon_ch1_file_service::IconCh1FileService;


pub trait MeteoSwissDiContainer: Send + Sync {
    fn get_icon_ch1_file_service(&mut self) -> &IconCh1FileService;
}
