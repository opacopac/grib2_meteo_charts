use crate::di_container::DiContainer;
use crate::dwd::dwd_di_container::DwdDiContainer;
use crate::dwd::test_dwd_di_container::TestDwdDiContainer;
use crate::meteo_swiss::meteo_swiss_di_container::MeteoSwissDiContainer;
use crate::meteo_swiss::test_meteo_swiss_di_container::TestMeteoSwissDiContainer;


pub struct TestDiContainer {
    dwd_di_container: Option<Box<dyn DwdDiContainer>>,
    meteo_swiss_di_container: Option<Box<dyn MeteoSwissDiContainer>>,
}


impl TestDiContainer {
    pub fn new() -> Self {
        Self {
            dwd_di_container: None,
            meteo_swiss_di_container: None,
        }
    }
}


impl DiContainer for TestDiContainer {
    fn get_dwd_di_container(&mut self) -> &Box<dyn DwdDiContainer> {
        if self.dwd_di_container.is_none() {
            self.dwd_di_container = Some(Box::new(TestDwdDiContainer::new()));
        }

        self.dwd_di_container.as_ref().unwrap()
    }


    fn get_meteo_swiss_di_container(&mut self) -> &Box<dyn MeteoSwissDiContainer> {
        if self.meteo_swiss_di_container.is_none() {
            self.meteo_swiss_di_container = Some(Box::new(TestMeteoSwissDiContainer::new()));
        }

        self.meteo_swiss_di_container.as_ref().unwrap()
    }
}
