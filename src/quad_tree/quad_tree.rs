use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::quad_tree::quad_tree_item::QuadTreeItem;
use crate::quad_tree::quad_tree_node::QuadTreeNode;

pub struct QuadTree<T> {
    pub max_node_capacity: usize,
    pub max_depth: usize,
    pub root_node: QuadTreeNode<T>
}


impl <T> QuadTree<T> {
    pub fn new(
        max_extent: LatLonExtent,
        max_node_capacity: usize,
        max_depth: usize
    ) -> QuadTree<T> {
        let root_node = QuadTreeNode::new(max_extent);
        let tree = QuadTree { max_node_capacity, max_depth, root_node };

        return tree;
    }


    pub fn add_item(&mut self, item: QuadTreeItem<T>) {
        if !self.root_node.extent.is_inside(&item.coord) {
            panic!("item has a position {:?} outside of max extent", item.coord);
        }

        self.root_node.add_item(item, self.max_node_capacity, self.max_depth);
    }


    pub fn count_items(&self) -> usize {
        return self.root_node.cound_items();
    }


    pub fn count_nodes(&self) -> usize {
        return self.root_node.count_nodes();
    }


    pub fn find_closest_item(&self, pos: &LatLon) -> Option<&QuadTreeItem<T>> {
        return self.root_node.find_closest_item(pos);
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;
    use crate::quad_tree::quad_tree::QuadTree;
    use crate::quad_tree::quad_tree_item::QuadTreeItem;

    #[test]
    fn it_creates_a_new_tree_with_an_empty_root_node() {
        let max_depth = 6;
        let max_node_capacity = 2;
        let max_extent = LatLonExtent::MAX_EXTENT;

        let tree: QuadTree<u32> = QuadTree::new(max_extent, max_node_capacity, max_depth);

        assert_eq!(max_depth, tree.max_depth);
        assert_eq!(max_node_capacity, tree.max_node_capacity);

        let root = tree.root_node;

        assert_eq!(0, root.items.len());
        assert_eq!(LatLonExtent::MAX_EXTENT, root.extent);
    }


    #[test]
    fn it_adds_a_new_item() {
        let mut tree: QuadTree<u32> = QuadTree::new(LatLonExtent::MAX_EXTENT, 10, 6);
        let item1 = QuadTreeItem::new(LatLon::new(1.0, 1.0), 1);
        let item2 = QuadTreeItem::new(LatLon::new(2.0, 2.0), 2);

        tree.add_item(item1);
        tree.add_item(item2);

        assert_eq!(2, tree.root_node.items.len());
    }


    #[test]
    #[should_panic]
    fn it_panics_when_adding_an_item_outside_of_the_max_extent() {
        let max_extent = LatLonExtent::new(LatLon::new(0.0, 0.0), LatLon::new(10.0, 10.0));
        let mut tree: QuadTree<u32> = QuadTree::new(max_extent, 10, 6);
        let item = QuadTreeItem::new(LatLon::new(-1.0, 1.0), 1);

        tree.add_item(item);
    }


    #[test]
    fn it_finds_the_closest_item_from_a_pos() {
        let mut tree: QuadTree<u32> = QuadTree::new(LatLonExtent::MAX_EXTENT, 10, 6);
        let item1 = QuadTreeItem::new(LatLon::new(1.0, 1.0), 1);
        let item2 = QuadTreeItem::new(LatLon::new(2.0, 2.0), 2);

        tree.add_item(item1);
        tree.add_item(item2);

        let result = tree.find_closest_item(&LatLon::new(-1.6, -1.6));
        assert!(result.is_some());

        let item = result.unwrap();
        assert_eq!(1, item.value);
    }


    #[test]
    fn it_gets_the_total_number_of_items() {
        let mut tree: QuadTree<u32> = QuadTree::new(LatLonExtent::MAX_EXTENT, 10, 6);
        let item1 = QuadTreeItem::new(LatLon::new(1.0, 1.0), 1);
        let item2 = QuadTreeItem::new(LatLon::new(-2.0, 2.0), 2);
        let item3 = QuadTreeItem::new(LatLon::new(3.0, -3.0), 3);

        let count1 = tree.count_items();
        assert_eq!(0, count1);

        tree.add_item(item1);
        tree.add_item(item2);
        tree.add_item(item3);

        let count2 = tree.count_items();
        assert_eq!(3, count2);
    }


    #[test]
    fn it_returns_the_total_number_of_nodes() {
        let mut tree: QuadTree<u32> = QuadTree::new(LatLonExtent::MAX_EXTENT, 1, 6);
        let item1 = QuadTreeItem::new(LatLon::new(1.0, 1.0), 1);
        let item2 = QuadTreeItem::new(LatLon::new(-2.0, 2.0), 2);
        let item3 = QuadTreeItem::new(LatLon::new(3.0, -3.0), 3);

        let count1 = tree.count_nodes();
        assert_eq!(1, count1);

        tree.add_item(item1);
        tree.add_item(item2);
        tree.add_item(item3);

        let count2 = tree.count_nodes();
        assert_eq!(4, count2);
    }
}
