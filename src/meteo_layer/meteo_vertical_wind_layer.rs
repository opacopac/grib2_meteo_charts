use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;


pub struct MeteoVerticalWindLayer<'a> {
    hhl_grids: &'a Vec<LatLonValueGrid<u8>>,
    u_grids: Vec<LatLonValueGrid<u8>>,
    v_grids: Vec<LatLonValueGrid<u8>>,
}


impl<'a> MeteoVerticalWindLayer<'a> {
    pub fn new(
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
        u_grids: Vec<LatLonValueGrid<u8>>,
        v_grids: Vec<LatLonValueGrid<u8>>,
    ) -> MeteoVerticalWindLayer<'_> {
        if hhl_grids.len() == 0 || u_grids.len() == 0 || v_grids.len() == 0 {
            panic!("number of hhl, v or u grids must not be null"); // TODO: return error
        }


        if hhl_grids.len() != u_grids.len() || hhl_grids.len() != v_grids.len() {
            panic!("number of hhl grids ({}), u grids ({}) and v grids ({}) must be the same", hhl_grids.len(), u_grids.len(), v_grids.len()); // TODO: return error
        }

        MeteoVerticalWindLayer { hhl_grids, u_grids, v_grids }
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize, usize) {
        let (x, y) = self.u_grids.get(0).unwrap().get_grid_dimensions();
        let level = self.u_grids.len();

        (x, y, level)
    }


    pub fn get_lat_lon_extent(&self) -> &LatLonExtent {
        self.u_grids.get(0).unwrap().get_grid_lat_lon_extent()
    }


    pub fn get_hhl_value(&self, x: usize, y: usize, level: usize) -> Option<u8> {
        self.hhl_grids[level].get_value_by_xy(x, y)
    }


    pub fn get_u_v_values(&self, x: usize, y: usize, level: usize) -> Option<(u8, u8)> {
        let u_opt = self.u_grids[level].get_value_by_xy(x, y);
        let v_opt = self.v_grids[level].get_value_by_xy(x, y);

        if u_opt.is_none() || v_opt.is_none() {
            None
        } else {
            Some((u_opt.unwrap(), v_opt.unwrap()))
        }
    }
}
