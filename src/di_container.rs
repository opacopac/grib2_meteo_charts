use crate::dwd::dwd_di_container::DwdDiContainer;
use crate::meteo_swiss::meteo_swiss_di_container::MeteoSwissDiContainer;


pub struct DiContainer {
    dwd_di_container: DwdDiContainer,
    meteo_swiss_di_container: MeteoSwissDiContainer,
}


impl DiContainer {
    pub fn create_productive() -> Self {
        Self {
            dwd_di_container: DwdDiContainer::create_productive(),
            meteo_swiss_di_container: MeteoSwissDiContainer::create_productive(),
        }
    }


    pub fn create_mock() -> Self {
        Self {
            dwd_di_container: DwdDiContainer::create_mock(),
            meteo_swiss_di_container: MeteoSwissDiContainer::create_mock(),
        }
    }


    pub fn get_dwd_di_container(&self) -> &DwdDiContainer {
        &self.dwd_di_container
    }


    pub fn get_meteo_swiss_di_container(&self) -> &MeteoSwissDiContainer {
        &self.meteo_swiss_di_container
    }
}
