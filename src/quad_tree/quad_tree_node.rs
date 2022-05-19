use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::quad_tree::quad_tree_item::QuadTreeItem;

pub struct QuadTreeNode<T> {
    pub extent: LatLonExtent,
    pub items: Vec<QuadTreeItem<T>>,
    pub child_nodes: Vec<QuadTreeNode<T>>
}


impl <T> QuadTreeNode<T> {
    pub fn new(extent: LatLonExtent) -> QuadTreeNode<T> {
        let items = vec![];
        let child_nodes = vec![];
        let node = QuadTreeNode { extent, items, child_nodes };

        return node;
    }


    pub fn add_item(&mut self, item: QuadTreeItem<T>, max_capacity: usize, max_depth: usize) {
        if self.child_nodes.len() == 0 {
            self.items.push(item);

            if self.items.len() > max_capacity && max_depth > 0 {
                self.child_nodes = self.create_child_nodes();
                while let Some(item) = self.items.pop() {
                    self.pass_to_child_node(item, max_capacity, max_depth - 1);
                }
            }
        } else {
            self.pass_to_child_node(item, max_capacity, max_depth - 1);
        }
    }


    fn create_child_nodes(&self) -> Vec<QuadTreeNode<T>> {
        let mid_point = self.extent.calc_midpoint();
        let extent1 = LatLonExtent::new(
            LatLon { lat: self.extent.min_coord.lat, lon: self.extent.min_coord.lon },
            LatLon { lat: mid_point.lat, lon: mid_point.lon }
        );
        let extent2 = LatLonExtent::new(
            LatLon { lat: self.extent.min_coord.lat, lon: mid_point.lon },
            LatLon { lat: mid_point.lat, lon: self.extent.max_coord.lon }
        );
        let extent3 = LatLonExtent::new(
            LatLon { lat: mid_point.lat, lon: self.extent.min_coord.lon },
            LatLon { lat: self.extent.max_coord.lat, lon: mid_point.lon }
        );
        let extent4 = LatLonExtent::new(
            LatLon { lat: mid_point.lat, lon: mid_point.lon },
            LatLon { lat: self.extent.max_coord.lat, lon: self.extent.max_coord.lon }
        );

        let mut child_nodes = vec![];
        child_nodes.push(QuadTreeNode::new(extent1));
        child_nodes.push(QuadTreeNode::new(extent2));
        child_nodes.push(QuadTreeNode::new(extent3));
        child_nodes.push(QuadTreeNode::new(extent4));

        return child_nodes;
    }


    fn pass_to_child_node(&mut self, item: QuadTreeItem<T>, max_capacity: usize, max_depth: usize) {
        for i in 0..self.child_nodes.len() {
            if self.child_nodes[i].extent.is_inside(&item.coord) {
                self.child_nodes[i].add_item(item, max_capacity, max_depth);
                break;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;
    use crate::quad_tree::quad_tree_item::QuadTreeItem;
    use crate::quad_tree::quad_tree_node::QuadTreeNode;

    #[test]
    fn it_creates_an_empty_node() {
        let node: QuadTreeNode<u32> = QuadTreeNode::new(LatLonExtent::MAX_EXTENT);

        assert_eq!(0, node.items.len());
        assert_eq!(LatLonExtent::MAX_EXTENT, node.extent);
    }


    #[test]
    fn it_adds_a_new_item() {
        let mut node: QuadTreeNode<u32> = QuadTreeNode::new(LatLonExtent::MAX_EXTENT);
        let item1 = QuadTreeItem::new(LatLon::new(1.0, 1.0), 1);
        let item2 = QuadTreeItem::new(LatLon::new(2.0, 2.0), 2);

        node.add_item(item1, 10, 6);
        node.add_item(item2, 10, 6);

        assert_eq!(2, node.items.len());
    }


    #[test]
    fn it_splits_into_4_child_nodes_when_reaching_max_capacity() {
        let mut node: QuadTreeNode<u32> = QuadTreeNode::new(LatLonExtent::MAX_EXTENT);
        let item1 = QuadTreeItem::new(LatLon::new(-1.0, -1.0), 1);
        let item2 = QuadTreeItem::new(LatLon::new(-2.0, 2.0), 2);
        let item3 = QuadTreeItem::new(LatLon::new(3.0, -3.0), 3);
        let item4 = QuadTreeItem::new(LatLon::new(4.0, 4.0), 4);
        let max_capacity = 3;
        let max_depth = 6;

        node.add_item(item1, max_capacity, max_depth);
        node.add_item(item2, max_capacity, max_depth);
        node.add_item(item3, max_capacity, max_depth);

        assert_eq!(3, node.items.len());
        assert_eq!(0, node.child_nodes.len());

        node.add_item(item4, max_capacity, max_depth);

        assert_eq!(0, node.items.len());
        assert_eq!(4, node.child_nodes.len());

        assert_eq!(LatLon { lat: -90.0, lon: -180.0 }, node.child_nodes[0].extent.min_coord);
        assert_eq!(LatLon { lat: 0.0, lon: 0.0 }, node.child_nodes[0].extent.max_coord);
        assert_eq!(1, node.child_nodes[0].items.len());
        assert_eq!(1, node.child_nodes[0].items[0].value);

        assert_eq!(LatLon { lat: -90.0, lon: 0.0 }, node.child_nodes[1].extent.min_coord);
        assert_eq!(LatLon { lat: 0.0, lon: 180.0 }, node.child_nodes[1].extent.max_coord);
        assert_eq!(1, node.child_nodes[1].items.len());
        assert_eq!(2, node.child_nodes[1].items[0].value);

        assert_eq!(LatLon { lat: 0.0, lon: -180.0 }, node.child_nodes[2].extent.min_coord);
        assert_eq!(LatLon { lat: 90.0, lon: 0.0 }, node.child_nodes[2].extent.max_coord);
        assert_eq!(1, node.child_nodes[2].items.len());
        assert_eq!(3, node.child_nodes[2].items[0].value);

        assert_eq!(LatLon { lat: 0.0, lon: 0.0 }, node.child_nodes[3].extent.min_coord);
        assert_eq!(LatLon { lat: 90.0, lon: 180.0 }, node.child_nodes[3].extent.max_coord);
        assert_eq!(1, node.child_nodes[3].items.len());
        assert_eq!(4, node.child_nodes[3].items[0].value);
    }


    #[test]
    fn it_has_a_max_depth() {
        let mut node: QuadTreeNode<u32> = QuadTreeNode::new(LatLonExtent::MAX_EXTENT);
        let item1 = QuadTreeItem::new(LatLon::new(-90.0, -180.0), 1);
        let item2 = QuadTreeItem::new(LatLon::new(-90.0, -180.0), 2);
        let max_capacity = 1;
        let max_depth = 2;

        node.add_item(item1, max_capacity, max_depth);

        assert_eq!(1, node.items.len());
        assert_eq!(0, node.child_nodes.len());

        node.add_item(item2, max_capacity, max_depth);

        assert_eq!(0, node.items.len());
        assert_eq!(4, node.child_nodes.len());

        assert_eq!(0, node.child_nodes[0].items.len());
        assert_eq!(0, node.child_nodes[1].items.len());
        assert_eq!(0, node.child_nodes[2].items.len());
        assert_eq!(0, node.child_nodes[3].items.len());
        assert_eq!(4, node.child_nodes[0].child_nodes.len());
        assert_eq!(0, node.child_nodes[1].child_nodes.len());
        assert_eq!(0, node.child_nodes[2].child_nodes.len());
        assert_eq!(0, node.child_nodes[3].child_nodes.len());

        let child_lv2 = &node.child_nodes[0];
        assert_eq!(2, child_lv2.child_nodes[0].items.len());
        assert_eq!(0, child_lv2.child_nodes[1].items.len());
        assert_eq!(0, child_lv2.child_nodes[2].items.len());
        assert_eq!(0, child_lv2.child_nodes[3].items.len());
        assert_eq!(0, child_lv2.child_nodes[0].child_nodes.len());
        assert_eq!(0, child_lv2.child_nodes[1].child_nodes.len());
        assert_eq!(0, child_lv2.child_nodes[2].child_nodes.len());
        assert_eq!(0, child_lv2.child_nodes[3].child_nodes.len());
    }
}
