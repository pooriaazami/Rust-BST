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
}
