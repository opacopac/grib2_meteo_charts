use crate::imaging::drawable::Drawable;
use crate::map_tile::map_tile_renderer::MapTileRenderer;
use crate::meteo_chart::forecast_renderer::map_tile_file_helper::MapTileFileHelper;
use crate::meteo_chart::forecast_renderer::meteo_chart_error::MeteoChartError;
use crate::meteo_chart::forecast_renderer::single_chart_renderer::SingleChartRenderer;
use crate::meteo_chart::meteo_layer::meteo_wind_10m_layer::MeteoWind10mLayer;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;


pub struct Wind10mChartRenderer;


impl Wind10mChartRenderer {
    const KNOTS_TO_MPS: f32 = 0.514444;


    pub fn render_full_chart(wind_layer: &MeteoWind10mLayer) -> Result<Drawable, MeteoChartError> {
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
        wind_layer: &MeteoWind10mLayer,
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


    pub fn render_map_tiles2(
        wind_layer: &MeteoWind10mLayer,
        fc_run: &MeteoForecastRun2,
        step_nr: usize,
    ) -> Result<(), MeteoChartError> {
        let extent = wind_layer.get_lat_lon_extent();
        let save_fn = |tile: &Drawable, zoom: u32, x: u32, y: u32| MapTileFileHelper::save_tile_step(
            tile, zoom, x, y, &wind_layer.get_type(), fc_run, step_nr,
        );

        let _ = MapTileRenderer::create_all_tiles(
            extent,
            fc_run.get_model().get_zoom_range(),
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
