use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::quad_tree::quad_tree::QuadTree;
use crate::quad_tree::quad_tree_item::QuadTreeItem;

pub struct UnstructuredGrid {
    quad_tree: QuadTree<usize>,
}


impl UnstructuredGrid {
    const MAX_NODE_CAPACITY: usize = 50;
    const MAX_TREE_DEPTH: usize = 10;


    pub fn new() -> UnstructuredGrid {
        let quad_tree = QuadTree::new(
            LatLonExtent::MAX_EXTENT,
            Self::MAX_NODE_CAPACITY,
            Self::MAX_TREE_DEPTH
        );
        let grid = UnstructuredGrid { quad_tree };

        return grid;
    }


    pub fn add_point_value(&mut self, lat_lon: LatLon, value: usize) {
        let item = QuadTreeItem::new(lat_lon, value);
        self.quad_tree.add_item(item);
    }


    pub fn get_point_count(&self) -> usize {
        return self.quad_tree.get_item_count();
    }


    pub fn get_value_by_lat_lon(&self, lat_lon: &LatLon) -> usize {
        let result = self.quad_tree.find_closest_item(lat_lon);
        let item = result.unwrap(); // TODO;

        return item.value;
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::unstructured_grid::UnstructuredGrid;

    #[test]
    fn it_adds_a_grid_point() {
        let mut grid = UnstructuredGrid::new();
        let lat_lon = LatLon::new(47.0, 7.0);

        grid.add_point_value(lat_lon, 1);

        assert_eq!(1, grid.quad_tree.get_item_count());
    }


    #[test]
    fn it_gets_the_number_of_grid_point() {
        let mut grid = UnstructuredGrid::new();
        grid.add_point_value(LatLon::new(47.0, 7.0), 1);
        grid.add_point_value(LatLon::new(47.6, 7.6), 2);

        let point_count = grid.get_point_count();

        assert_eq!(2, point_count);
    }


    #[test]
    fn it_gets_the_point_value_by_lat_lon() {
        let mut grid = UnstructuredGrid::new();
        grid.add_point_value(LatLon::new(47.0, 7.0), 1);
        grid.add_point_value(LatLon::new(47.6, 7.6), 2);
        let lat_lon = LatLon::new(48.0, 8.0);

        let result = grid.get_value_by_lat_lon(&lat_lon);

        assert_eq!(2, result);
    }
}
