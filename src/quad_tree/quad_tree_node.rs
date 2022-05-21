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
        if self.child_nodes.len() > 0 {
            self.pass_to_child_node(item, max_capacity, max_depth - 1);
        } else {
            self.items.push(item);

            if self.items.len() > max_capacity && max_depth > 0 {
                self.child_nodes = self.create_child_nodes();
                while let Some(item) = self.items.pop() {
                    self.pass_to_child_node(item, max_capacity, max_depth - 1);
                }
            }
        }
    }


    pub fn cound_items(&self) -> usize {
        if self.child_nodes.len() == 0 {
            return self.items.len();
        }

        let mut count = 0;
        for child_node in &self.child_nodes {
            count += child_node.cound_items();
        }

        return count;
    }


    pub fn count_nodes(&self) -> usize {
        if self.child_nodes.len() == 0 {
            return 1;
        }

        let mut count = 0;
        for child_node in &self.child_nodes {
            count += child_node.count_nodes();
        }

        return count;
    }


    pub fn find_closest_item(&self, pos: &LatLon) -> Option<&QuadTreeItem<T>> {
        if self.child_nodes.len() > 0 {
            match self.find_containing_child_index(pos) {
                Some(idx) => {
                    let item = self.child_nodes[idx].find_closest_item(pos);
                    if item.is_some() {
                        return item;
                    }

                    let item = self.find_closes_item_in_adjacent_children(pos, idx);
                    if item.is_some() {
                        return item;
                    }
                },
                None => {}
            }
        }

        return self.find_closest_item_of_self(pos);
    }


    fn find_containing_child_index(&self, pos: &LatLon) -> Option<usize> {
        for i in 0..self.child_nodes.len() {
            if self.child_nodes[i].extent.is_inside(pos) {
                return Some(i);
            }
        }

        return None;
    }


    fn find_closes_item_in_adjacent_children(&self, pos: &LatLon, child_idx: usize) -> Option<&QuadTreeItem<T>> {
        let mut child_matches: Vec<Option<&QuadTreeItem<T>>> = vec![];

        match child_idx {
            // bottom left
            0 => {
                let pos1 = LatLon::new(pos.lat, self.child_nodes[1].extent.min_coord.lon);
                let pos2 = LatLon::new(self.child_nodes[2].extent.min_coord.lon, pos.lon);
                let pos3 = LatLon::new(self.child_nodes[3].extent.min_coord.lat, self.child_nodes[3].extent.min_coord.lon);

                child_matches.push(self.child_nodes[1].find_closest_item(&pos1));
                child_matches.push(self.child_nodes[2].find_closest_item(&pos2));
                child_matches.push(self.child_nodes[3].find_closest_item(&pos3));
            },
            // bottom right
            1 => {
                let pos0 = LatLon::new(pos.lat, self.child_nodes[0].extent.max_coord.lon);
                let pos3 = LatLon::new(self.child_nodes[3].extent.min_coord.lon, pos.lon);
                let pos2 = LatLon::new(self.child_nodes[2].extent.min_coord.lat, self.child_nodes[2].extent.max_coord.lon);

                child_matches.push(self.child_nodes[0].find_closest_item(&pos0));
                child_matches.push(self.child_nodes[3].find_closest_item(&pos3));
                child_matches.push(self.child_nodes[2].find_closest_item(&pos2));
            },
            // top left
            2 => {
                let pos3 = LatLon::new(pos.lat, self.child_nodes[3].extent.min_coord.lon);
                let pos0 = LatLon::new(self.child_nodes[0].extent.max_coord.lon, pos.lon);
                let pos1 = LatLon::new(self.child_nodes[1].extent.max_coord.lat, self.child_nodes[1].extent.min_coord.lon);

                child_matches.push(self.child_nodes[3].find_closest_item(&pos3));
                child_matches.push(self.child_nodes[0].find_closest_item(&pos0));
                child_matches.push(self.child_nodes[1].find_closest_item(&pos1));
            }
            // top right
            _ => {
                let pos2 = LatLon::new(pos.lat, self.child_nodes[2].extent.max_coord.lon);
                let pos1 = LatLon::new(self.child_nodes[1].extent.max_coord.lon, pos.lon);
                let pos0 = LatLon::new(self.child_nodes[0].extent.max_coord.lat, self.child_nodes[0].extent.max_coord.lon);

                child_matches.push(self.child_nodes[2].find_closest_item(&pos2));
                child_matches.push(self.child_nodes[1].find_closest_item(&pos1));
                child_matches.push(self.child_nodes[0].find_closest_item(&pos0));
            }
        }

        let item_candidates: Vec<&QuadTreeItem<T>> = child_matches.iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();

        if item_candidates.len() == 0 {
            return None;
        }

        let mut best_dist = -1.0;
        let mut best_idx = 0;
        for i in 0..item_candidates.len() {
            let dist = item_candidates[i].coord.calc_square_euclidean_dist(&pos);
            if dist < best_dist || best_dist < 0.0 {
                best_dist = dist;
                best_idx = i;
            }
        }

        let closest_item = item_candidates[best_idx];

        return Some(closest_item);
    }


    fn find_closest_item_of_self(&self, pos: &LatLon) -> Option<&QuadTreeItem<T>> {
        if self.items.len() == 0 {
            return None;
        }

        let mut best_dist = -1.0;
        let mut best_idx = 0;
        for i in 0..self.items.len() {
            let dist = self.items[i].coord.calc_square_euclidean_dist(&pos);

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
        // bottom left
        let extent1 = LatLonExtent::new(
            LatLon { lat: self.extent.min_coord.lat, lon: self.extent.min_coord.lon },
            LatLon { lat: mid_point.lat, lon: mid_point.lon }
        );
        // bottom right
        let extent2 = LatLonExtent::new(
            LatLon { lat: self.extent.min_coord.lat, lon: mid_point.lon },
            LatLon { lat: mid_point.lat, lon: self.extent.max_coord.lon }
        );
        // top left
        let extent3 = LatLonExtent::new(
            LatLon { lat: mid_point.lat, lon: self.extent.min_coord.lon },
            LatLon { lat: self.extent.max_coord.lat, lon: mid_point.lon }
        );
        // top right
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
