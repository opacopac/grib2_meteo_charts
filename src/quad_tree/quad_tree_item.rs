use derive_new::new;

use crate::geo::lat_lon::LatLon;

#[derive(new)]
pub struct QuadTreeItem<T> {
    pub coord: LatLon,
    pub value: T
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::quad_tree::quad_tree_item::QuadTreeItem;

    #[test]
    fn it_stores_a_coordinate_and_a_value() {
        let coord = LatLon::new(47.0, 7.0);
        let coord2 = coord.clone();
        let value: usize = 99;

        let result = QuadTreeItem::new(coord, value);

        assert_eq!(coord2, result.coord);
        assert_eq!(value, result.value);
    }
}
