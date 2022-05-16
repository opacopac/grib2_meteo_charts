use crate::geo::lat_lon_extent::LatLonExtent;
use crate::quad_tree::quad_tree_node::QuadTreeNode;

pub struct QuadTree {
    pub root_node: QuadTreeNode
}


impl QuadTree {
    pub fn new(extent: LatLonExtent) -> QuadTree {
        let root_node = QuadTreeNode::new(extent);
        let tree = QuadTree { root_node };

        return tree;
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;
    use crate::quad_tree::quad_tree::QuadTree;

    #[test]
    fn it_creates_a_new_tree_with_an_empty_root_node() {
        let extent = LatLonExtent::new(
            LatLon::new(-90.0, -180.0),
            LatLon::new(90.0, 180.0)
        );
        let tree = QuadTree::new(extent);
        let root = tree.root_node;

        assert_eq!(0, root.items.len());
    }
}
