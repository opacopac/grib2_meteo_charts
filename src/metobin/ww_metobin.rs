use crate::meteo_dwd::dwd_ww_layer::DwdWwLayer;

pub struct WwMeteoBin {
    ww_layer: DwdWwLayer
}


impl WwMeteoBin {
    const NONE_BIN_VALUE: u8 = 0xFF;


    pub fn new(ww_layer: DwdWwLayer) -> WwMeteoBin {
        return WwMeteoBin { ww_layer };
    }


    pub fn create_bin_values(&self) -> Vec<u8> {
        let dim = self.ww_layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let result = self.ww_layer.get_ww_by_xy(x, y);
                let out_val = match result {
                    Some(val_ww) => val_ww.to_value(),
                    None => Self::NONE_BIN_VALUE
                };

                out_values.push(out_val);
            }
        }

        return out_values;
    }
}
