use crate::imaging::drawable::Drawable;
use crate::meteo_chart::forecast_renderer::meteo_chart_error::MeteoChartError;


pub struct SingleChartRenderer;


impl SingleChartRenderer {
    pub fn render<T>(
        width: u32,
        height: u32,
        value_fn: impl Fn(usize, usize) -> Option<T>,
        color_fn: impl Fn(T) -> [u8; 4],
    ) -> Result<Drawable, MeteoChartError> {
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

        Ok(drawable)
    }
}
