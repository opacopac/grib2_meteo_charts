use crate::geo::lat_lon::LatLon;
use crate::geo::map_tile_grid::MapTileGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section0::discipline::Discipline;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
use crate::meteo::cloud_layer::CloudLayer;
use crate::meteo_dwd::discipline_checker::DisciplineChecker;
use crate::meteo_dwd::value_grid::ValueGrid;

pub struct DwdIconGlobalTotalCloudCoverLayer {
    pub parameter_category: MeteoParameterCategory,
    pub parameter_number: u8,
    pub grid: MapTileGrid<usize, 4096>,
    pub data_points: Vec<f32>
}


impl DwdIconGlobalTotalCloudCoverLayer {
    pub fn create(doc: Grib2Document, grid: MapTileGrid<usize, 4096>) -> Result<DwdIconGlobalTotalCloudCoverLayer, Grib2Error> {
        DisciplineChecker::check(
            &doc,
            Discipline::Meteorological,
            MeteoParameterCategory::Cloud,
            199
        )?;

        let parameter_cat_num = DisciplineChecker::get_parameter_category_number(&doc)?;
        let data_points = doc.calculate_data_points(ValueGrid::MISSING_VALUE)?;
        let layer = DwdIconGlobalTotalCloudCoverLayer {
            parameter_category: parameter_cat_num.0,
            parameter_number: parameter_cat_num.1,
            grid,
            data_points
        };


        return Ok(layer);
    }
}


impl CloudLayer for DwdIconGlobalTotalCloudCoverLayer {
    fn get_latlon_grid_points(&self) -> (u32, u32) {
        return (4096 as u32, 4096 as u32);
    }


    fn get_tot_cloud_cover_percent_by_latlon(&self, pos: &LatLon) -> f32 {
        todo!()
    }


    fn get_tot_cloud_cover_percent_by_index(&self, index: usize) -> f32 {
        let y = index / 4096;
        let x = index - (y * 4096);

        let idx = self.grid.get_value(x, y);

        return self.data_points[idx];
    }
}
