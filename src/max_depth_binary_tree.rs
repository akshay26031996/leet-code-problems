// https://leetcode.com/problems/maximum-depth-of-binary-tree/

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => {
                return 0;
            }
            Some(node) => {
                return 1 + max(depth(&node.borrow().left), depth(&node.borrow().right));
            }
        }
    }
    depth(&root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_depth_binary_tree_basic() {
        let mut root = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let mut r = TreeNode::new(20);
        r.left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        r.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.right = Some(Rc::new(RefCell::new(r)));
        assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), 3);
    }
}
