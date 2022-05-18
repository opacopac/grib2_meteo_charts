use crate::geo::lat_lon_extent::LatLonExtent;

pub struct QuadTreeNode {
    pub extent: LatLonExtent,
    pub items: Vec<QuadTreeNode>
}


impl QuadTreeNode {
    const MAX_CAPACITY: usize = 100;


    pub fn new(extent: LatLonExtent) -> QuadTreeNode {
        let items = vec![];
        let node = QuadTreeNode { extent, items };

        return node;
    }


    pub fn get_capacity() -> usize {
        return Self::MAX_CAPACITY;
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon_extent::LatLonExtent;
    use crate::quad_tree::quad_tree_node::QuadTreeNode;


    #[test]
    fn it_creates_an_empty_node() {
        let extent = LatLonExtent::MAX_EXTENT;

        let node = QuadTreeNode::new(extent);

        assert_eq!(0, node.items.len());
        assert_eq!(LatLonExtent::MAX_EXTENT, node.extent);
    }


    #[test]
    fn it_has_a_max_capacity() {
        let capacity = QuadTreeNode::get_capacity();

        assert_eq!(100, capacity);
    }
}
