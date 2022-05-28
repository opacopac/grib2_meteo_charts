use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;

pub struct SingleChartRenderer;

impl SingleChartRenderer {
    pub fn render(
        width: u32,
        height: u32,
        value_fn: impl Fn(usize, usize) -> Option<f32>,
        color_fn: impl Fn(f32) -> [u8; 4]
    ) -> Result<Drawable, Grib2Error> {
        let mut drawable = Drawable::create_empty(width, height)?;

        for i in 0..height {
            for j in 0..width {
                let value = value_fn(j as usize, i as usize); // TODO
                match value {
                    Some(v) => {
                        let color = color_fn(v);
                        drawable.draw_point(j, height - i - 1, color);
                    }
                    _ => continue
                }
            }
        }

        return Ok(drawable);
    }
}
