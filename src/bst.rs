pub mod bst {
    use std::cmp::Ordering;

    #[derive(Debug, Eq, PartialEq, PartialOrd)]
    pub struct BSTNode<T>
    where
        T: Eq + PartialEq + Ord + PartialOrd,
    {
        data: T,
        left_node: Box<Option<BSTNode<T>>>,
        right_node: Box<Option<BSTNode<T>>>,
    }

    pub struct BST<T>
    where
        T: Eq + PartialEq + Ord + PartialOrd,
    {
        root: BSTNode<T>,
    }

    impl<T> BSTNode<T>
    where
        T: Eq + PartialEq + Ord + PartialOrd,
    {
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

    impl<T> Ord for BSTNode<T>
    where
        T: Eq + PartialEq + Ord + PartialOrd,
    {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.get_data() > other.get_data() {
                Ordering::Greater
            } else if self.get_data() < other.get_data() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    }

    impl<T> BST<T>
    where
        T: Eq + PartialEq + Ord + PartialOrd,
    {
        fn traverse(&self, goal: &BSTNode<T>) -> &BSTNode<T> {
            todo!()
        }

        fn add() {
            todo!()
        }

        fn remove() {
            todo!()
        }

        fn contains() {
            todo!()
        }

        fn size() {
            todo!()
        }

        fn hight() {
            todo!()
        }
    }
}
