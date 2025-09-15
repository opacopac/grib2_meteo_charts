use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::meteo_chart::map_tile_renderer::MapTileRenderer;
use crate::meteo_chart::single_chart_renderer::SingleChartRenderer;
use crate::meteo_layer::meteo_temp_layer::MeteoTempLayer;


pub struct TempChartRenderer;


impl TempChartRenderer {
    pub fn render_full_chart(temp_layer: &MeteoTempLayer) -> Result<Drawable, Grib2Error> {
        let dimensions = temp_layer.get_grid_dimensions();
        let drawable = SingleChartRenderer::render(
            dimensions.0 as u32,
            dimensions.1 as u32,
            |x, y| temp_layer.get_temp_by_xy(x, y),
            |value| Self::color_fn(value),
        )?;

        Ok(drawable)
    }


    pub fn render_map_tiles<S>(
        temp_layer: &MeteoTempLayer,
        zoom_range: (u32, u32),
        save_fn: S,
    ) -> Result<(), Grib2Error>
    where
        S: Fn(&Drawable, u32, u32, u32) -> () + Sync,
    {
        let extent = temp_layer.get_lat_lon_extent();

        MapTileRenderer::create_all_tiles(
            extent,
            zoom_range,
            |pos| temp_layer.get_temp_by_lat_lon(pos),
            |value| Self::color_fn(value),
            save_fn,
        )
    }


    fn color_fn(value_k: f32) -> [u8; 4] {
        const TRANSPARENCY: u8 = 127; // 50% transparency
        const KELVIN_OFFSET: f32 = 273.15;

        let value_c = value_k - KELVIN_OFFSET;

        // todo: lower/higher values?
        if value_c < -12.0 {
            [255, 153, 255, TRANSPARENCY] // pink
        } else if value_c < -10.0 {
            [255, 204, 255, TRANSPARENCY] // white pink
        } else if value_c < -8.0 {
            [255, 255, 255, TRANSPARENCY] // white
        } else if value_c < -6.0 {
            [219, 235, 250, TRANSPARENCY] // white blue
        } else if value_c < -4.0 {
            [0, 212, 245, TRANSPARENCY] // cyan
        } else if value_c < -2.0 {
            [148, 189, 240, TRANSPARENCY] // light blue
        } else if value_c < 0.0 {
            [115, 166, 235, TRANSPARENCY] // cornflower blue
        } else if value_c < 2.0 {
            [222, 230, 153, TRANSPARENCY] // light olive-yellow
        } else if value_c < 4.0 {
            [166, 212, 115, TRANSPARENCY] // light olive
        } else if value_c < 6.0 {
            [107, 191, 77, TRANSPARENCY] // light green
        } else if value_c < 8.0 {
            [51, 171, 38, TRANSPARENCY] // medium green
        } else if value_c < 10.0 {
            [0, 153, 0, TRANSPARENCY] // dark green
        } else if value_c < 12.0 {
            [51, 179, 0, TRANSPARENCY] // dark lime
        } else if value_c < 14.0 {
            [102, 204, 0, TRANSPARENCY] // medium lime
        } else if value_c < 16.0 {
            [153, 230, 0, TRANSPARENCY] // lime
        } else if value_c < 18.0 {
            [204, 255, 0, TRANSPARENCY] // light lime
        } else if value_c < 20.0 {
            [255, 255, 0, TRANSPARENCY] // yellow
        } else if value_c < 22.0 {
            [255, 204, 0, TRANSPARENCY] // light orange
        } else if value_c < 24.0 {
            [255, 153, 0, TRANSPARENCY] // orange
        } else if value_c < 26.0 {
            [255, 102, 0, TRANSPARENCY] // dark orange
        } else if value_c < 28.0 {
            [255, 51, 0, TRANSPARENCY] // orange-red
        } else if value_c < 30.0 {
            [255, 0, 0, TRANSPARENCY] // red
        } else if value_c < 32.0 {
            [255, 0, 255, TRANSPARENCY] // magenta
        } else if value_c < 34.0 {
            [255, 64, 255, TRANSPARENCY] // medium magenta
        } else if value_c < 36.0 {
            [255, 128, 255, TRANSPARENCY] // light magenta
        } else if value_c < 38.0 {
            [255, 191, 255, TRANSPARENCY] // rose
        } else {
            [255, 255, 255, TRANSPARENCY] // white
        }
    }
}
