use crate::geo::lat_lon_extent::LatLonExtent;
use crate::quad_tree::quad_tree_node::QuadTreeNode;

pub struct QuadTree {
    pub root_node: QuadTreeNode
}


impl QuadTree {
    pub fn new() -> QuadTree {
        let root_node = QuadTreeNode::new(LatLonExtent::MAX_EXTENT);
        let tree = QuadTree { root_node };

        return tree;
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon_extent::LatLonExtent;
    use crate::quad_tree::quad_tree::QuadTree;

    #[test]
    fn it_creates_a_new_tree_with_an_empty_root_node_and_a_max_extent() {
        let tree = QuadTree::new();

        let root = tree.root_node;

        assert_eq!(0, root.items.len());
        assert_eq!(LatLonExtent::MAX_EXTENT, root.extent);
    }
}
