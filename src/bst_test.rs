#[cfg(test)]
mod bst_test {
    use crate::bst::bst::{BSTNode, BST};

    #[test]
    fn constudtion_test() {
        let node = BSTNode::new(10);

        assert_eq!(node.get_data(), &10);
        assert_eq!(node.left(), Option::<&BSTNode::<i32>>::None);
        assert_eq!(node.right(), Option::<&BSTNode::<i32>>::None);
    }

    #[test]
    fn test_bst_node_ordering() {
        let one_1 = BSTNode::new(1);
        let one_2 = BSTNode::new(1);

        let two_1 = BSTNode::new(2);
        let three_1 = BSTNode::new(3);

        assert_eq!(three_1 > two_1, true);
        assert_eq!(two_1 > one_1, true);
        assert_eq!(one_1 == one_2, true);

        assert_eq!(one_1 < two_1, true);
        assert_eq!(two_1 < three_1, true);

        assert_eq!(three_1 < two_1, false);
        assert_eq!(two_1 < one_1, false);

        assert_eq!(one_1 > two_1, false);
        assert_eq!(two_1 > three_1, false);

        assert_eq!(one_1 == two_1, false);
    }

    #[test]
    fn test_build_and_contains() {
        let mut tree = BST::new(10);

        assert_eq!(tree.contains(&10), true);
        assert_eq!(tree.contains(&9), false);
    }

    #[test]
    fn test_add() {
        let mut tree = BST::new(10);

        assert_eq!(tree.contains(&5), false);
        tree.add(5);
        assert_eq!(tree.contains(&5), true);

        assert_eq!(tree.contains(&15), false);
        tree.add(15);
        assert_eq!(tree.contains(&15), true);

        assert_eq!(tree.contains(&2), false);
        tree.add(2);
        assert_eq!(tree.contains(&2), true);

        assert_eq!(tree.contains(&7), false);
        tree.add(7);
        assert_eq!(tree.contains(&7), true);

        assert_eq!(tree.contains(&12), false);
        tree.add(12);
        assert_eq!(tree.contains(&12), true);

        assert_eq!(tree.contains(&17), false);
        tree.add(17);
        assert_eq!(tree.contains(&17), true);
    }

    #[test]
    fn test_hight() {
        let mut tree = BST::new(10);

        assert_eq!(tree.hight(), 1);

        tree.add(5);
        assert_eq!(tree.hight(), 2);

        tree.add(15);
        assert_eq!(tree.hight(), 2);

        tree.add(2);
        assert_eq!(tree.hight(), 3);

        tree.add(7);
        assert_eq!(tree.hight(), 3);

        tree.add(12);
        assert_eq!(tree.hight(), 3);

        tree.add(20);
        assert_eq!(tree.hight(), 3);

        tree.add(30);
        assert_eq!(tree.hight(), 4);

        tree.add(40);
        assert_eq!(tree.hight(), 5);

        tree.add(50);
        assert_eq!(tree.hight(), 6);

        tree.add(60);
        assert_eq!(tree.hight(), 7);

        tree.add(25);
        assert_eq!(tree.hight(), 7);
    }
}
