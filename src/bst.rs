pub mod bst {

    pub struct BSTNode<T> {
        data: T,
        left_node: Box<Option<BSTNode<T>>>,
        right_node: Box<Option<BSTNode<T>>>,
    }

    pub struct BST<T> {
        root: BSTNode<T>,
    }

    impl<T> BSTNode<T> {
        fn new(data: T) -> BSTNode<T> {
            BSTNode {
                data: data,
                left_node: Box::new(None),
                right_node: Box::new(None),
            }
        }
    }
}
