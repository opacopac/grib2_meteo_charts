use crate::grib2::common::lat_lon::LatLon;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section3::grid_definition_template::GridDefinitionTemplate;
use crate::grib2::section5::data_representation_template::DataRepresentationTemplate::GridPointDataSimplePacking;

pub struct CloudCoverLayer {
    pub document: Grib2Document
}


impl CloudCoverLayer {
    pub fn new(
        document: Grib2Document
    ) -> CloudCoverLayer {
        return CloudCoverLayer {
            document
        };
    }


    pub fn get_value_by_index(&self, index: usize) -> f32 {
        let raw_value = self.document.section7.data_points[index];
        println!("raw value {:?}", raw_value);

        match &self.document.section5.data_representation_template {
            GridPointDataSimplePacking(tpl) => {
                println!("{:?}", tpl);
                let c1 = (2 as f32).powi(tpl.binary_scale_factor_e as i32);
                let c2 = (10 as f32).powi(tpl.decimal_scale_factor_d as i32);
                /*let c1 = Math.pow(2, section5.data.dataRepresentationTemplate.E)
                let c2 = Math.pow(10, section5.data.dataRepresentationTemplate.D)*/

                return (tpl.reference_value + raw_value as f32 * c1) as f32 / c2;
            }
            _ => { panic!("invalid data represenation template") } // TODO: temp
        }

        // return (self.section5.data_representation_template.data.dataRepresentationTemplate.R + rawValue * c1) / c2;
    }


    pub fn get_value_by_lat_lon(&self, pos: LatLon) -> f32 {
        // The grid is stored by column and not by row!!!
        /*const lonIndex = Math.round(((lon - section3.data.gridDefinitionTemplate.Lo1) / section3.data.gridDefinitionTemplate.jInc))
        const latIndex = Math.round(((lat - section3.data.gridDefinitionTemplate.La1) / section3.data.gridDefinitionTemplate.iInc))

        var bestIndex = latIndex * section3.data.gridDefinitionTemplate.numberOfPointsAlongParallel
        bestIndex += lonIndex*/

        match &self.document.section3.grid_definition_template {
            GridDefinitionTemplate::LatLon(tpl) => {
                println!("{:?}", tpl);

                let lon_idx = ((pos.lon - &tpl.first_grid_point.lon) / &tpl.j_direction_increment).round() as u32;
                let lat_idx = ((pos.lat - &tpl.first_grid_point.lat) / &tpl.i_direction_increment).round() as u32;
                let idx = (lat_idx * &tpl.number_of_points_along_parallel + lon_idx) as usize;
                let value = self.get_value_by_index(idx);

                println!("lon_idx: {} lat_idx: {} idx: {} value: {}", lon_idx, lat_idx, idx, value);

                return value;

            }
            _ => { panic!("invalid grid definition template") } // TODO: temp
        }
    }
}
