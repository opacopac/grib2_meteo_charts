use crate::imaging::drawable::Drawable;
use crate::map_tile::map_tile_renderer::MapTileRenderer;
use crate::meteo_chart::meteo_chart_error::MeteoChartError;
use crate::meteo_chart::single_chart_renderer::SingleChartRenderer;
use crate::meteo_layer::meteo_wind_layer::MeteoWindLayer;


pub struct WindChartRenderer;


impl WindChartRenderer {
    const KNOTS_TO_MPS: f32 = 0.514444;


    pub fn render_full_chart(wind_layer: &MeteoWindLayer) -> Result<Drawable, MeteoChartError> {
        let dimensions = wind_layer.get_grid_dimensions();
        let drawable = SingleChartRenderer::render(
            dimensions.0 as u32,
            dimensions.1 as u32,
            |x, y| wind_layer.get_wind_speed_tot_xy(x, y),
            |value| Self::color_fn(value),
        )?;

        Ok(drawable)
    }


    pub fn render_map_tiles<S>(
        wind_layer: &MeteoWindLayer,
        zoom_range: (u32, u32),
        save_fn: S,
    ) -> Result<(), MeteoChartError> where
        S: Fn(&Drawable, u32, u32, u32) -> () + Sync,
    {
        let extent = wind_layer.get_lat_lon_extent();

        let _ = MapTileRenderer::create_all_tiles(
            extent,
            zoom_range,
            |pos| wind_layer.get_wind_speed_tot_by_lat_lon(pos),
            |value| Self::color_fn(value),
            save_fn,
        )?;

        Ok(())
    }


    fn color_fn(value: f32) -> [u8; 4] {
        /*if value < 2.5 * Self::KNOTS_TO_MPS {
            [0, 0, 0, 0] // transparent
        } else*/ if value < 5.0 * Self::KNOTS_TO_MPS {
            [0, 127, 0, 255] // dark green
        } else if value < 10.0 * Self::KNOTS_TO_MPS {
            [0, 255, 0, 255] // light green
        } else if value < 15.0 * Self::KNOTS_TO_MPS {
            [255, 255, 0, 255] // yellow
        } else if value < 20.0 * Self::KNOTS_TO_MPS {
            [255, 191, 0, 255] // light orange
        } else if value < 25.0 * Self::KNOTS_TO_MPS {
            [255, 128, 0, 255] // dark orange
        } else if value < 30.0 * Self::KNOTS_TO_MPS {
            [255, 0, 0, 255] // red
        } else if value < 40.0 * Self::KNOTS_TO_MPS {
            [163, 73, 164, 255] // purple
        } else if value < 50.0 * Self::KNOTS_TO_MPS {
            [99, 112, 247, 255] // light blue
        } else {
            [0, 0, 255, 255] // blue
        }
    }
}
