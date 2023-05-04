use bst::bst::BST;

pub mod bst;
mod bst_test;

fn main() {
    let mut tree = BST::new(10);

    tree.add(5);
    tree.add(15);
    tree.add(2);
    tree.add(7);
    tree.add(12);
    tree.add(17);

    tree.print_tree();
}
