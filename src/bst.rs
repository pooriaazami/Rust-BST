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

        fn borrow_inside_the_box<'a>(
            &'a self,
            node: &'a Box<Option<BSTNode<T>>>,
        ) -> Option<&BSTNode<T>> {
            match node.as_ref() {
                Some(node) => Some(node),
                None => None,
            }
        }

        pub fn left(&self) -> Option<&BSTNode<T>> {
            self.borrow_inside_the_box(&self.left_node)
        }

        pub fn right(&self) -> Option<&BSTNode<T>> {
            self.borrow_inside_the_box(&self.right_node)
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
        fn traverse(&self, goal: &BSTNode<T>) -> Option<&BSTNode<T>> {
            let mut current_node = Some(&self.root);

            while let Some(node) = current_node {
                if node.get_data() == goal.get_data() {
                    return Some(node);
                }

                if goal.get_data() < node.get_data() {
                    current_node = node.left();
                } else {
                    current_node = node.right();
                }
            }

            None
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
