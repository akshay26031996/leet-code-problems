// https://leetcode.com/problems/binary-tree-inorder-traversal/

use std::cell::RefCell;
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

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node) = node {
            traverse(&node.borrow().left, res);
            res.push(node.borrow().val);
            traverse(&node.borrow().right, res);
        }
    }
    traverse(&root, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inorder_traversal_basic() {
        let mut root = TreeNode::new(1);
        let mut root_right = TreeNode::new(2);
        let root_right_left = TreeNode::new(3);
        root_right.left = Some(Rc::new(RefCell::new(root_right_left)));
        root.right = Some(Rc::new(RefCell::new(root_right)));
        assert_eq!(
            inorder_traversal(Some(Rc::new(RefCell::new(root)))),
            vec![1, 3, 2]
        );
    }
}
