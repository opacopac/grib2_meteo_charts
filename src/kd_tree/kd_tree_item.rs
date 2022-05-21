use derive_new::new;

use crate::geo::lat_lon::LatLon;

#[derive(new)]
pub struct KdTreeItem<T> {
    pub coord: LatLon,
    pub value: T
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::kd_tree::kd_tree_item::KdTreeItem;

    #[test]
    fn it_creates_an_instance_with_an_item_an_a_coord() {
        let coord = LatLon::new(47.0, 7.0);
        let coord_expected = coord.clone();
        let value = 5;
        let value_expected = value;

        let result = KdTreeItem::new(coord, value);

        assert_eq!(coord_expected, result.coord);
        assert_eq!(value_expected, result.value);
    }
}
