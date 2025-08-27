use crate::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;


pub struct VerticalWindMeteobin<'a> {
    layer: MeteoVerticalWindLayer<'a>,
}


impl<'a> VerticalWindMeteobin<'a> {
    const MISSING_VALUE: u8 = 255;


    pub fn new(
        layer: MeteoVerticalWindLayer
    ) -> VerticalWindMeteobin {
        VerticalWindMeteobin { layer }
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

                    let u_v_values = self.layer.get_u_v_values(x, y, level);
                    if u_v_values.is_some() {
                        out_values.push(u_v_values.unwrap().0);
                        out_values.push(u_v_values.unwrap().1);
                    } else {
                        out_values.push(Self::MISSING_VALUE);
                        out_values.push(Self::MISSING_VALUE);
                    }
                }
            }
        }

        out_values
    }
}
