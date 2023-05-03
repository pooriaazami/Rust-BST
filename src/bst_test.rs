#[cfg(test)]
mod bst_test {
    use crate::bst::bst::BSTNode;

    #[test]
    fn constudtion_test() {
        let node = BSTNode::new(10);

        assert_eq!(node.get_data(), &10);
        assert_eq!(node.left(), &Box::new(Option::<BSTNode::<i32>>::None));
        assert_eq!(node.right(), &Box::new(Option::<BSTNode::<i32>>::None));
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
}
