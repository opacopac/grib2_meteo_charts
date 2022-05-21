use crate::kd_tree::kd_tree_node::KdTreeNode;

pub struct KdTree<T> {
    root_node: Option<KdTreeNode<T>>
}


impl <T> KdTree<T> {
    pub fn new() -> KdTree<T> {
        let tree = KdTree { root_node: None };

        return tree;
    }


    /*pub fn add_item(&self, item: KdTreeItem<T>) {
        let root_node: KdTreeNode<T>::new(item);
    }*/
}


#[cfg(test)]
mod tests {
    use crate::kd_tree::kd_tree::KdTree;

    #[test]
    fn it_creates_an_empty_kd_tree() {
        let tree: KdTree<()> = KdTree::new();

        assert!(tree.root_node.is_none());
    }


    /*#[test]
    fn it_adds_an_item_to_the_tree() {
        let item = KdTreeItem::new(LatLon::new(47.0, 7.0), 1);
        let tree = KdTree::new();

        tree.add_item(item);

        assert!(tree.root_node.is_some());
    }*/


    /*#[test]
    fn it_calculates_the_median_item_and_left_right_list() {
        let mut items = vec![];
        items.push(KdTreeItem::new(LatLon::new(47.0, 7.0), 1));
        items.push(KdTreeItem::new(LatLon::new(48.0, 8.0), 2));
        items.push(KdTreeItem::new(LatLon::new(49.0, 9.0), 3));

        let (median, left, right) = KdTree::calc_item_median(items);

        assert_eq!(2, median.value);
        assert_eq!(1, left.len());
        assert_eq!(1, left[0].value);
        assert_eq!(1, right.len());
        assert_eq!(3, right[0].value);
    }*/
}
