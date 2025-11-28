use crate::dwd::dwd_di_container::DwdDiContainer;
use crate::dwd::dwd_file_reader::icon_d2_file_service::IconD2FileService;


pub struct TestDwdDiContainer {}


impl TestDwdDiContainer {
    pub fn new() -> Self {
        Self {}
    }
}


impl DwdDiContainer for TestDwdDiContainer {
    fn get_icon_d2_file_service(&mut self) -> &IconD2FileService {
        todo!("not implemented yet")
    }
}
