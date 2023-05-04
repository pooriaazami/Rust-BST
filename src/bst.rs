pub mod bst {
    use std::cmp::Ordering;

    #[derive(Debug, Eq, PartialEq, PartialOrd)]
    pub struct BSTNode<T>
    where
        T: Eq + PartialEq + Ord + PartialOrd,
    {
        data: T,
        left_node: Option<Box<BSTNode<T>>>,
        right_node: Option<Box<BSTNode<T>>>,
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
                left_node: None,
                right_node: None,
            }
        }

        pub fn get_data(&self) -> &T {
            &self.data
        }

        fn borrow_inside_the_box<'a>(
            &'a self,
            node: &'a Option<Box<BSTNode<T>>>,
        ) -> Option<&BSTNode<T>> {
            match node.as_ref() {
                Some(node) => Some(node),
                None => None,
            }
        }

        pub fn left(&self) -> Option<&BSTNode<T>> {
            self.borrow_inside_the_box(&self.left_node)
        }

        pub fn mut_left(&mut self) -> Option<&mut BSTNode<T>> {
            let node = &mut self.left_node;
            match node {
                Some(node) => Some(node),
                None => None,
            }
        }

        pub fn right(&self) -> Option<&BSTNode<T>> {
            self.borrow_inside_the_box(&self.right_node)
        }

        pub fn mut_right(&mut self) -> Option<&mut BSTNode<T>> {
            let node = &mut self.right_node;
            match node {
                Some(node) => Some(node),
                None => None,
            }
        }

        pub fn add_data(&mut self, data: T) {
            if data == self.data {
                return;
            }

            if data < self.data {
                let child = BSTNode::new(data);
                self.left_node = Some(Box::new(child));
            } else {
                let child = BSTNode::new(data);
                self.right_node = Some(Box::new(child));
            }
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
        fn traverse(&self, goal: &T) -> Option<&BSTNode<T>> {
            let mut current_node = Some(&self.root);

            while let Some(node) = current_node {
                if node.get_data() == goal {
                    return Some(node);
                }

                if goal < node.get_data() {
                    current_node = node.left();
                } else {
                    current_node = node.right();
                }
            }

            None
        }

        pub fn add(&mut self, data: T) {
            let mut parent = &mut self.root;
            loop {
                if parent.get_data() == &data {
                    return;
                }

                if parent.get_data() > &data {
                    if parent.left() != None {
                        parent = parent.mut_left().unwrap();
                    } else {
                        parent.add_data(data);
                        return;
                    }
                } else {
                    if parent.right() != None {
                        parent = parent.mut_right().unwrap();
                    } else {
                        parent.add_data(data);
                        return;
                    }
                }
            }
        }

        fn remove(&mut self) {
            todo!()
        }

        fn contains(&self, data: &T) -> bool {
            match self.traverse(data) {
                Some(_) => true,
                None => false,
            }
        }

        fn size(&self) {
            todo!()
        }

        fn hight(&self) {
            todo!()
        }
    }
}
