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


    pub fn cound_items(&self) -> usize {
        return if self.child_nodes.len() == 0 {
            self.items.len()
        } else {
            let mut count = 0;
            for child_node in &self.child_nodes {
                count += child_node.cound_items();
            }

            count
        }
    }


    pub fn count_nodes(&self) -> usize {
        return if self.child_nodes.len() == 0 {
            1
        } else {
            let mut count = 0;
            for child_node in &self.child_nodes {
                count += child_node.count_nodes();
            }

            count
        }
    }


    pub fn find_closest_item(&self, pos: &LatLon) -> Option<&QuadTreeItem<T>> {
        if self.child_nodes.len() > 0 {
            let mut closest_node: Option<&QuadTreeItem<T>> = None;
            for child_node in &self.child_nodes {
                if child_node.extent.is_inside(pos) {
                    closest_node = child_node.find_closest_item(pos);
                    break;
                }
            }

            if closest_node.is_some() {
                return closest_node;
            }
        }

        return self.find_closest_item_of_self(pos);
    }


    fn find_closest_item_of_self(&self, pos: &LatLon) -> Option<&QuadTreeItem<T>> {
        if self.items.len() == 0 {
            return None;
        }

        let mut best_dist = -1.0;
        let mut best_idx = 0;
        for i in 0..self.items.len() {
            let item = &self.items[i];
            let lat_diff = item.coord.lat - pos.lat;
            let lon_diff = item.coord.lon - pos.lon;
            let dist = lat_diff * lat_diff + lon_diff * lon_diff;

            if dist < best_dist || best_dist < 0.0 {
                best_dist = dist;
                best_idx = i;
            }
        }

        let closest_item = &self.items[best_idx];

        return Some(closest_item);
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


    #[test]
    fn it_gets_the_total_number_of_items() {
        let mut node: QuadTreeNode<u32> = QuadTreeNode::new(LatLonExtent::MAX_EXTENT);
        let item1 = QuadTreeItem::new(LatLon::new(1.0, 1.0), 1);
        let item2 = QuadTreeItem::new(LatLon::new(-2.0, 2.0), 2);
        let item3 = QuadTreeItem::new(LatLon::new(3.0, -3.0), 3);

        let count = node.cound_items();
        assert_eq!(0, count);

        node.add_item(item1, 2, 6);

        let count = node.cound_items();
        assert_eq!(1, count);

        node.add_item(item2, 2, 6);

        let count = node.cound_items();
        assert_eq!(2, count);

        node.add_item(item3, 2, 6);

        let count = node.cound_items();
        assert_eq!(3, count);
    }


    #[test]
    fn it_finds_the_closest_item_from_a_pos() {
        let mut node: QuadTreeNode<u32> = QuadTreeNode::new(LatLonExtent::MAX_EXTENT);
        let item1 = QuadTreeItem::new(LatLon::new(-1.0, -1.0), 1);
        let item2 = QuadTreeItem::new(LatLon::new(-2.0, 2.0), 2);
        let item3 = QuadTreeItem::new(LatLon::new(3.0, -3.0), 3);
        let item4 = QuadTreeItem::new(LatLon::new(4.0, 4.0), 4);
        let item5 = QuadTreeItem::new(LatLon::new(5.0, 5.0), 5);
        let max_capacity = 3;
        let max_depth = 6;

        node.add_item(item1, max_capacity, max_depth);
        node.add_item(item2, max_capacity, max_depth);
        node.add_item(item3, max_capacity, max_depth);
        node.add_item(item4, max_capacity, max_depth);
        node.add_item(item5, max_capacity, max_depth);

        let result = node.find_closest_item(&LatLon::new(4.9, 4.8));
        assert!(result.is_some());

        let item = result.unwrap();
        assert_eq!(5, item.value);
    }


    #[test]
    fn it_finds_the_closest_item_from_a_pos_in_an_adjacent_quadrant() {
        let mut node: QuadTreeNode<u32> = QuadTreeNode::new(LatLonExtent::MAX_EXTENT);
        let item1 = QuadTreeItem::new(LatLon::new(-1.0, -1.0), 1);
        let item2 = QuadTreeItem::new(LatLon::new(1.0, 1.0), 2);
        let max_capacity = 1;
        let max_depth = 6;

        node.add_item(item1, max_capacity, max_depth);
        node.add_item(item2, max_capacity, max_depth);

        let result = node.find_closest_item(&LatLon::new(-1.0, 0.5));
        assert!(result.is_some());

        let item = result.unwrap();
        assert_eq!(1, item.value);
    }


    #[test]
    fn it_returns_none_when_searching_an_empty_node() {
        let node: QuadTreeNode<u32> = QuadTreeNode::new(LatLonExtent::MAX_EXTENT);

        let result = node.find_closest_item(&LatLon::new(4.9, 4.8));
        assert!(result.is_none());
    }


    #[test]
    fn it_returns_the_number_of_nodes() {
        let mut node: QuadTreeNode<u32> = QuadTreeNode::new(LatLonExtent::MAX_EXTENT);
        let item1 = QuadTreeItem::new(LatLon::new(-1.0, -1.0), 1);
        let item2 = QuadTreeItem::new(LatLon::new(-2.0, 2.0), 2);
        let item3 = QuadTreeItem::new(LatLon::new(3.0, -3.0), 3);
        let item4 = QuadTreeItem::new(LatLon::new(4.0, 4.0), 4);
        let item5 = QuadTreeItem::new(LatLon::new(55.0, 95.0), 5);
        let max_capacity = 1;
        let max_depth = 6;

        let node_count = node.count_nodes();
        assert_eq!(1, node_count);

        node.add_item(item1, max_capacity, max_depth);
        let node_count = node.count_nodes();
        assert_eq!(1, node_count);

        node.add_item(item2, max_capacity, max_depth);
        let node_count = node.count_nodes();
        assert_eq!(4, node_count);

        node.add_item(item3, max_capacity, max_depth);
        let node_count = node.count_nodes();
        assert_eq!(4, node_count);

        node.add_item(item4, max_capacity, max_depth);
        let node_count = node.count_nodes();
        assert_eq!(4, node_count);

        node.add_item(item5, max_capacity, max_depth);
        let node_count = node.count_nodes();
        assert_eq!(7, node_count);
    }
}
