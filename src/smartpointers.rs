pub fn smart_pointers() {
    // let b_int1 = Box::new(10);
    // println!("b_int1 = {}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        //chances where they don't have a left or right.
        pub key: T,
    }
    //rust doesn't like null values, and if there is no additional node or value, thats going to trigger an error, but you can use box to fix it.

    impl<T> TreeNode<T> {
        pub fn new_node(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }
    //create root node and assign left and right at one time.

    // let node1 = TreeNode::new(1)
    //     .left(TreeNode::new(2))
    //     .right(TreeNode::new(3));
}
