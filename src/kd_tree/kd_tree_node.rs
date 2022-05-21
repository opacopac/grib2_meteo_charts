use crate::geo::lat_lon::LatLon;
use crate::kd_tree::kd_tree_item::KdTreeItem;

pub struct KdTreeNode<T> {
    item: KdTreeItem<T>,
    left_child: Box<Option<KdTreeNode<T>>>,
    right_child: Box<Option<KdTreeNode<T>>>
}


impl <T> KdTreeNode<T> {
    pub fn new(item: KdTreeItem<T>) -> KdTreeNode<T> {
        return KdTreeNode { item, left_child: Box::new(None), right_child: Box::new(None) };
    }


    pub fn add_item(&mut self, item: KdTreeItem<T>, use_lat: bool) {
        let is_left = self.check_is_left_child(&item.coord, use_lat);
        if is_left {
            match self.left_child.as_mut() {
                Some(child) => child.add_item(item, !use_lat),
                None => self.left_child = Box::new(Some(KdTreeNode::new(item)))
            }
        } else {
            match self.right_child.as_mut() {
                Some(child) => child.add_item(item, !use_lat),
                None => self.right_child = Box::new(Some(KdTreeNode::new(item)))
            }
        }
    }


    fn check_is_left_child(&self, coord: &LatLon, use_lat: bool) -> bool {
        return if use_lat {
            coord.lat < self.item.coord.lat
        } else {
            coord.lon < self.item.coord.lon
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::kd_tree::kd_tree_item::KdTreeItem;
    use crate::kd_tree::kd_tree_node::KdTreeNode;

    #[test]
    fn it_creates_an_instance_with_one_item_and_no_children() {
        let item = KdTreeItem::new(LatLon::new(47.0, 7.0), 99);

        let result = KdTreeNode::new(item);

        assert_eq!(99, result.item.value);
        assert!(result.left_child.is_none());
        assert!(result.right_child.is_none());
    }


    #[test]
    fn it_adds_an_item_to_the_right_and_left_child() {
        let item1 = KdTreeItem::new(LatLon::new(47.0, 7.0), 1);
        let item2 = KdTreeItem::new(LatLon::new(48.0, 8.0), 2);
        let item3 = KdTreeItem::new(LatLon::new(49.0, 9.0), 3);
        let mut node = KdTreeNode::new(item2);

        node.add_item(item1, true);
        node.add_item(item3, true);

        match node.left_child.as_ref() {
            Some(x) => assert_eq!(1, x.item.value),
            None => panic!()
        }

        match node.right_child.as_ref() {
            Some(x) => assert_eq!(3, x.item.value),
            None => panic!()
        }
    }


    #[test]
    fn it_adds_an_item_by_lon() {
        let item1 = KdTreeItem::new(LatLon::new(47.0, 8.0), 1);
        let item2 = KdTreeItem::new(LatLon::new(48.0, 7.0), 2);
        let mut node = KdTreeNode::new(item1);

        node.add_item(item2, false);

        match node.left_child.as_ref() {
            Some(x) => assert_eq!(2, x.item.value),
            None => panic!()
        }

        assert!(node.right_child.is_none());
    }


    #[test]
    fn it_alternates_lat_lon_for_each_level() {
        let item1 = KdTreeItem::new(LatLon::new(47.0, 9.0), 1);
        let item2 = KdTreeItem::new(LatLon::new(48.0, 8.0), 2);
        let item3 = KdTreeItem::new(LatLon::new(49.0, 7.0), 3);
        let mut node = KdTreeNode::new(item1);

        node.add_item(item2, true);
        node.add_item(item3, true);

        assert!(node.left_child.is_none());

        match node.right_child.as_ref() {
            None => panic!(),
            Some(child_node) => {
                assert_eq!(2, child_node.item.value);

                assert!(child_node.right_child.is_none());

                match child_node.left_child.as_ref() {
                    Some(x) => assert_eq!(3, x.item.value),
                    None => panic!()
                }
            }
        }
    }
}
