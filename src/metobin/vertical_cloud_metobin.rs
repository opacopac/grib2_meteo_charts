use crate::meteo_layer::meteo_vertical_cloud_layer::MeteoVerticalCloudLayer;


pub struct VerticalCloudMeteobin {}


impl VerticalCloudMeteobin {
    const MISSING_VALUE: u8 = 255;


    pub fn create_bin_values(layer: &MeteoVerticalCloudLayer) -> Vec<u8> {
        let (dim_x, dim_y, dim_level) = layer.get_grid_dimensions();
        let mut out_values = vec![];
        for y in 0..dim_y {
            for x in 0..dim_x {
                for level in 0..dim_level {
                    let hhl_value = layer.get_hhl_value(x, y, level);
                    if hhl_value.is_some() {
                        out_values.push(hhl_value.unwrap());
                    } else {
                        out_values.push(Self::MISSING_VALUE);
                    }

                    let clc_value = layer.get_clc_value(x, y, level);
                    if clc_value.is_some() {
                        out_values.push(clc_value.unwrap());
                    } else {
                        out_values.push(Self::MISSING_VALUE);
                    }
                }
            }
        }

        out_values
    }
}
