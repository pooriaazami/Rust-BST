pub mod bst {

    #[derive(PartialEq, Eq, Debug)]
    pub struct BSTNode<T> {
        data: T,
        left_node: Box<Option<BSTNode<T>>>,
        right_node: Box<Option<BSTNode<T>>>,
    }

    pub struct BST<T> {
        root: BSTNode<T>,
    }

    impl<T> BSTNode<T> {
        pub fn new(data: T) -> BSTNode<T> {
            BSTNode {
                data: data,
                left_node: Box::new(None),
                right_node: Box::new(None),
            }
        }

        pub fn get_data(&self) -> &T {
            &self.data
        }

        pub fn left(&self) -> &Box<Option<BSTNode<T>>> {
            &self.left_node
        }

        pub fn right(&self) -> &Box<Option<BSTNode<T>>> {
            &self.right_node
        }
    }
}
