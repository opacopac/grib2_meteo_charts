use crate::chart::map_tile_renderer::MapTileRenderer;
use crate::chart::single_chart_renderer::SingleChartRenderer;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::meteo_dwd::dwd_cloud_precip_layer::DwdCloudPrecipLayer;

pub struct CloudPrecipChartRenderer;


impl CloudPrecipChartRenderer {
    pub fn render_full_chart(cloud_precip_layer: &DwdCloudPrecipLayer) -> Result<Drawable, Grib2Error> {
        let dimensions = cloud_precip_layer.get_grid_dimensions();
        let drawable = SingleChartRenderer::render(
            dimensions.0 as u32,
            dimensions.1 as u32,
            |x, y| cloud_precip_layer.get_cloud_and_precip_by_xy(x, y),
            |value| Self::color_fn(value)
        )?;

        return Ok(drawable);
    }


    pub fn render_map_tiles<S>(
        cloud_layer: &DwdCloudPrecipLayer,
        zoom_range: (u32, u32),
        save_fn: S
    ) -> Result<(), Grib2Error> where
        S: Fn(&Drawable, u32, u32, u32) -> () + Sync
    {
        let extent = cloud_layer.get_lat_lon_extent();

        MapTileRenderer::create_all_tiles(
            extent,
            zoom_range,
            |pos| cloud_layer.get_cloud_and_precip_by_lat_lon(pos),
            |value| Self::color_fn(value),
            save_fn
        )
    }


    fn color_fn(value: (f32, f32)) -> [u8; 4] {
        if value.0 == 0.0 && value.1 == 0.0 {
            return [0, 0, 0, 0];
        }

        let cloud_color = Self::get_cloud_color(value.0);
        let rain_color = Self::get_rain_color(value.1);
        let composite_color = [
            (rain_color[0] as u32 * rain_color[3] as u32 / 255 + cloud_color[0] as u32 * cloud_color[3] as u32 * (255 - rain_color[3] as u32) / 255 / 255) as u8,
            (rain_color[1] as u32 * rain_color[3] as u32 / 255 + cloud_color[1] as u32 * cloud_color[3] as u32 * (255 - rain_color[3] as u32) / 255 / 255) as u8,
            (rain_color[2] as u32 * rain_color[3] as u32 / 255 + cloud_color[2] as u32 * cloud_color[3] as u32 * (255 - rain_color[3] as u32) / 255 / 255) as u8,
            255,
        ];

        return composite_color;
    }


    fn get_rain_color(value: f32) -> [u8; 4] {
        return if value == 0.0 {
            [0, 0, 0, 0] // transparent
        } else if value < 1.0 {
            [0, 192, 255, 127] // light blue
        } else if value < 2.0 {
            [0, 0, 255, 127] // blue
        } else if value < 4.0 {
            [0, 127, 0, 127] // dark green
        } else if value < 6.0 {
            [0, 255, 0, 127] // light green
        } else if value < 10.0 {
            [255, 255, 0, 127] // yellow
        } else if value < 20.0 {
            [255, 191, 0, 127] // light orange
        } else if value < 40.0 {
            [255, 128, 0, 127] // dark orange
        } else if value < 60.0 {
            [255, 0, 0, 127] // red
        } else {
            [163, 73, 164, 127] // purple
        }
    }


    fn get_cloud_color(value: f32) -> [u8; 4] {
        /*let gloud_gray = (255.0 - value * 128.0).floor() as u8;

        return [gloud_gray, gloud_gray, gloud_gray, 255]*/

        let alpha = (127.0 + value * 128.0).floor() as u8;

        return [255, 255, 255, alpha];
    }
}
