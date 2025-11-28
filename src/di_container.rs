use crate::dwd::dwd_di_container::DwdDiContainer;
use crate::meteo_swiss::meteo_swiss_di_container::MeteoSwissDiContainer;


pub trait DiContainer: Send + Sync {
    fn get_dwd_di_container(&mut self) -> &Box<dyn DwdDiContainer>;

    fn get_meteo_swiss_di_container(&mut self) -> &Box<dyn MeteoSwissDiContainer>;
}
