use crate::dwd::dwd_di_container::{DwdDiContainer, ProdDwdDiContainer};
use crate::meteo_swiss::meteo_swiss_di_container::{MeteoSwissDiContainer, ProdMeteoSwissDiContainer};


pub trait DiContainer: Send + Sync {
    fn get_dwd_di_container(&mut self) -> &Box<dyn DwdDiContainer>;

    fn get_meteo_swiss_di_container(&mut self) -> &Box<dyn MeteoSwissDiContainer>;
}


pub struct ProdDiContainer {
    dwd_di_container: Option<Box<dyn DwdDiContainer>>,
    meteo_swiss_di_container: Option<Box<dyn MeteoSwissDiContainer>>,
}


impl ProdDiContainer {
    pub fn new() -> Self {
        Self {
            dwd_di_container: None,
            meteo_swiss_di_container: None,
        }
    }
}


impl DiContainer for ProdDiContainer {
    fn get_dwd_di_container(&mut self) -> &Box<dyn DwdDiContainer> {
        if self.dwd_di_container.is_none() {
            self.dwd_di_container = Some(Box::new(ProdDwdDiContainer::new()));
        }

        self.dwd_di_container.as_ref().unwrap()
    }


    fn get_meteo_swiss_di_container(&mut self) -> &Box<dyn MeteoSwissDiContainer> {
        if self.meteo_swiss_di_container.is_none() {
            self.meteo_swiss_di_container = Some(Box::new(ProdMeteoSwissDiContainer::new()));
        }

        self.meteo_swiss_di_container.as_ref().unwrap()
    }
}
