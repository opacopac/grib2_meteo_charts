use crate::meteo_layer::vertical_cloud_layer::DwdVerticalCloudLayer;

pub struct DwdVerticalCloudMeteobin<'a> {
    layer: DwdVerticalCloudLayer<'a>
}

impl <'a> DwdVerticalCloudMeteobin<'a> {
    const MISSING_VALUE: u8 = 255;

    pub fn new(
        layer: DwdVerticalCloudLayer
    ) -> DwdVerticalCloudMeteobin {
        return DwdVerticalCloudMeteobin { layer };
    }


    pub fn create_bin_values(&self) -> Vec<u8> {
        let (dim_x, dim_y, dim_level) = self.layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim_y {
            for x in 0..dim_x {
                for level in 0..dim_level {
                    let hhl_value = self.layer.get_hhl_value(x, y, level);
                    if hhl_value.is_some() {
                        out_values.push(hhl_value.unwrap());
                    } else {
                        out_values.push(Self::MISSING_VALUE);
                    }

                    let clc_value = self.layer.get_clc_value(x, y, level);
                    if clc_value.is_some() {
                        out_values.push(clc_value.unwrap());
                    } else {
                        out_values.push(Self::MISSING_VALUE);
                    }
                }
            }
        }

        return out_values;
    }
}


#[cfg(test)]
mod tests {
    /*#[test]
    fn it_calculates_the_bin_value_for_3kt() {
        let in_value = 3.0 / DwdWindMeteobin::KNOTS_PER_MPS;
        let result = DwdWindMeteobin::calc_speed_kt_value(in_value);

        assert_eq!(3 + 128, result);
    }


    #[test]
    fn it_limits_the_max_bin_value_to_plus127() {
        let in_value = 150.0 / DwdWindMeteobin::KNOTS_PER_MPS;
        let result = DwdWindMeteobin::calc_speed_kt_value(in_value);

        assert_eq!(254, result);
    }


    #[test]
    fn it_limits_the_min_bin_value_to_neg128() {
        let in_value = -200.0 / DwdWindMeteobin::KNOTS_PER_MPS;
        let result = DwdWindMeteobin::calc_speed_kt_value(in_value);

        assert_eq!(0 as u8, result);
    }*/
}
