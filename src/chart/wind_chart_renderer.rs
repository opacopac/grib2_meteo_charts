use crate::chart::map_tile_renderer::MapTileRenderer;
use crate::chart::single_chart_renderer::SingleChartRenderer;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::dwd_layer::dwd_wind_layer::DwdWindLayer;

pub struct WindChartRenderer;


impl WindChartRenderer {
    const KNOTS_TO_MPS: f32 = 0.514444;


    pub fn render_full_chart(wind_layer: &DwdWindLayer) -> Result<Drawable, Grib2Error> {
        let dimensions = wind_layer.get_grid_dimensions();
        let drawable = SingleChartRenderer::render(
            dimensions.0 as u32,
            dimensions.1 as u32,
            |x, y| wind_layer.get_wind_speed_tot_xy(x, y),
            |value| Self::color_fn(value)
        )?;

        return Ok(drawable);
    }


    pub fn render_map_tiles<S>(
        wind_layer: &DwdWindLayer,
        zoom_range: (u32, u32),
        save_fn: S
    ) -> Result<(), Grib2Error> where
        S: Fn(&Drawable, u32, u32, u32) -> () + Sync
    {
        let extent = wind_layer.get_lat_lon_extent();

        MapTileRenderer::create_all_tiles(
            extent,
            zoom_range,
            |pos| wind_layer.get_wind_speed_tot_by_lat_lon(pos),
            |value| Self::color_fn(value),
            save_fn
        )
    }


    fn color_fn(value: f32) -> [u8; 4] {
        return /*if value < 2.5 * Self::KNOTS_TO_MPS {
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
