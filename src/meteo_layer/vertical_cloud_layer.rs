use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;

pub struct DwdVerticalCloudLayer<'a> {
    hhl_grids: &'a Vec<LatLonValueGrid<u8>>,
    clc_grids: Vec<LatLonValueGrid<u8>>
}


impl <'a> DwdVerticalCloudLayer<'a> {
    pub fn new(
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
        clc_grids: Vec<LatLonValueGrid<u8>>
    ) -> DwdVerticalCloudLayer<'_> {
        if hhl_grids.len() == 0 || clc_grids.len() == 0 {
            panic!("number of hhl grids or clc grids must not be null"); // TODO: return error
        }


        if hhl_grids.len() != clc_grids.len() {
            panic!("number of hhl grids ({}) and clc grids ({}) must be the same", hhl_grids.len(), clc_grids.len()); // TODO: return error
        }

        return DwdVerticalCloudLayer { hhl_grids, clc_grids };
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize, usize) {
        let (x, y) = self.clc_grids.get(0).unwrap().get_grid_dimensions();
        let level = self.clc_grids.len();

        return (x, y, level);
    }


    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        return self.clc_grids.get(0).unwrap().get_grid_lat_lon_extent();
    }


    pub fn get_hhl_value(&self, x: usize, y: usize, level: usize) -> Option<u8> {
        return self.hhl_grids[level].get_value_by_xy(x, y);
    }


    pub fn get_clc_value(&self, x: usize, y: usize, level: usize) -> Option<u8> {
        return self.clc_grids[level].get_value_by_xy(x, y);
    }
}
