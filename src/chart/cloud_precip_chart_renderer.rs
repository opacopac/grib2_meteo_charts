use crate::chart::map_tile_renderer::MapTileRenderer;
use crate::chart::single_chart_renderer::SingleChartRenderer;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;

pub struct CloudPrecipChartRenderer;


impl CloudPrecipChartRenderer {
    pub fn render_full_chart(cloud_precip_layer: &MeteoCloudPrecipLayer) -> Result<Drawable, Grib2Error> {
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
        cloud_layer: &MeteoCloudPrecipLayer,
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
            return [0, 0, 0, 127];
        }

        let cloud_color = Self::get_cloud_color(value.0);
        let rain_color = Self::get_rain_color(value.1);
        let composite_color = [
            (rain_color[0] as u32 * rain_color[3] as u32 / 255 + cloud_color[0] as u32 * cloud_color[3] as u32 * (255 - rain_color[3] as u32) / 255 / 255) as u8,
            (rain_color[1] as u32 * rain_color[3] as u32 / 255 + cloud_color[1] as u32 * cloud_color[3] as u32 * (255 - rain_color[3] as u32) / 255 / 255) as u8,
            (rain_color[2] as u32 * rain_color[3] as u32 / 255 + cloud_color[2] as u32 * cloud_color[3] as u32 * (255 - rain_color[3] as u32) / 255 / 255) as u8,
            (rain_color[3] as u32 + cloud_color[3] as u32 * (255 - rain_color[3] as u32) / 255) as u8,
        ];

        return composite_color;
        /*return if rain_color[3] == 0 {
            cloud_color
        } else {
            rain_color
        }*/
    }


    fn get_rain_color(value: f32) -> [u8; 4] {
        return if value < 0.1 {
            [0, 0, 0, 0] // transparent
        } else if value < 1.0 {
            [0, 192, 255, 127] // light blue
        } else if value < 2.0 {
            [0, 0, 255, 191] // blue
        } else if value < 4.0 {
            [0, 127, 0, 191] // dark green
        } else if value < 6.0 {
            [0, 255, 0, 191] // light green
        } else if value < 10.0 {
            [255, 255, 0, 191] // yellow
        } else if value < 20.0 {
            [255, 191, 0, 191] // light orange
        } else if value < 40.0 {
            [255, 128, 0, 191] // dark orange
        } else if value < 60.0 {
            [255, 0, 0, 191] // red
        } else {
            [163, 73, 164, 191] // purple
        }
    }


    fn get_cloud_color(value: f32) -> [u8; 4] {
        /*let gloud_gray = (255.0 - value * 128.0).floor() as u8;

        return [gloud_gray, gloud_gray, gloud_gray, 255]*/

        /*let alpha = (127.0 + value * 128.0).floor() as u8;

        return [255, 255, 255, alpha];*/


        return if value < 0.05 {
            [0, 0, 0, 127] // transparent
        } else if value < 0.1 {
            [255, 255, 255, 127]
        } else if value < 0.2 {
            [255, 255, 255, 127]
        } else if value < 0.3 {
            [255, 255, 255, 127]
        } else if value < 0.4 {
            [255, 255, 255, 127]
        } else if value < 0.5 {
            [255, 255, 255, 159]
        } else if value < 0.6 {
            [255, 255, 255, 191]
        } else if value < 0.7 {
            [255, 255, 255, 207]
        } else if value < 0.8 {
            [255, 255, 255, 223]
        } else if value < 0.9 {
            [255, 255, 255, 239]
        } else {
            [255, 255, 255, 255]
        }
    }
}
