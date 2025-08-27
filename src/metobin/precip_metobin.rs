use crate::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;


pub struct PrecipMeteoBin {
    layer: MeteoCloudPrecipLayer
}


impl PrecipMeteoBin {
    const NONE_BIN_VALUE: u8 = 0xFF;


    pub fn new(cloud_precip_layer: MeteoCloudPrecipLayer) -> PrecipMeteoBin {
        PrecipMeteoBin { layer: cloud_precip_layer }
    }


    pub fn create_bin_values(&self) -> Vec<u8> {
        let dim = self.layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let result_cloud_precip = self.layer.get_cloud_and_precip_by_xy(x, y);
                let out_val_precip = Self::calc_precip_value(result_cloud_precip);

                out_values.push(out_val_precip);
            }
        }

        out_values
    }


    fn calc_precip_value(value_cloud_precip: Option<(f32, f32)>) -> u8 {
        match value_cloud_precip {
            Some(val) => {
                if val.1 >= 0.2 && val.1 < 0.5 {
                    1 // 0.5 * 2
                } else {
                    (val.1 * 2.0).round() as u8
                }
            }
            None => Self::NONE_BIN_VALUE
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::metobin::precip_metobin::PrecipMeteoBin;

    #[test]
    fn it_calculates_the_correct_precip_meteobin_values() {
        assert_eq!(0, PrecipMeteoBin::calc_precip_value(Some((0.0, 0.0))));
        assert_eq!(2, PrecipMeteoBin::calc_precip_value(Some((0.0, 1.0))));
        assert_eq!(1, PrecipMeteoBin::calc_precip_value(Some((0.0, 0.2))));
        assert_eq!(1, PrecipMeteoBin::calc_precip_value(Some((0.0, 0.5))));
        assert_eq!(1, PrecipMeteoBin::calc_precip_value(Some((0.0, 0.74))));
        assert_eq!(2, PrecipMeteoBin::calc_precip_value(Some((0.0, 0.75))));
        assert_eq!(72, PrecipMeteoBin::calc_precip_value(Some((0.0, 35.8))));
        assert_eq!(254, PrecipMeteoBin::calc_precip_value(Some((0.0, 127.0))));
        assert_eq!(0xFF, PrecipMeteoBin::calc_precip_value(None));
    }
}
