use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
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

    pub fn tree_node_wrap(node: TreeNode) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn new_left_right(val: i32, left: i32, right: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: Self::tree_node_wrap(Self::new(right)),
        }
    }

    pub fn new_left(val: i32, left: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: None,
        }
    }

    pub fn new_right(val: i32, right: i32) -> Self {
        let right = Self::new(right);
        TreeNode {
            val,
            left: None,
            right: Some(Rc::new(RefCell::new(right))),
        }
    }
}

/*
  - A valid BST is defined as follows:
    - The left subtree of a node contains only nodes
      with keys less than the node's key.
    - The right subtree of a node contains only nodes with
      keys greater than the node's key. Both the left and
      right subtrees must also be binary search trees.
*/

pub struct Solution {}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(_) = root {
            Solution::is_valid_bst_internal(&root)
        } else {
            true
        }
    }

    /*
      - if a subtree is empty it's valid
    */
    pub fn is_valid_bst_internal(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(refcell) = root {
            let node = refcell.borrow();
            let val = node.val;
            // check the left key
            let left_valid = if let Some(left_node_refcell) = &node.left {
                left_node_refcell.borrow().val < val
            } else {
                true
            };
            /* don't even bother checking the right */
            if !left_valid {
                return false;
            }
            // check the right key
            let right_valid = if let Some(right_node_refcell) = &node.right {
                right_node_refcell.borrow().val > val
            } else {
                true
            };

            /*
              - stop right here;
                no need to further traverse the tree
            */
            if !right_valid {
                return false;
            }
            /*
              - continue checking left and right
            */
            Solution::is_valid_bst_internal(&node.left)
                && Solution::is_valid_bst_internal(&node.right)
        } else {
            true
        }
    }

    pub fn tree_test_fixture_1() -> Option<Rc<RefCell<TreeNode>>> {
        TreeNode::tree_node_wrap(TreeNode::new_left_right(2, 1, 3))
    }

    pub fn tree_test_fixture_2() -> Option<Rc<RefCell<TreeNode>>> {
        let l = TreeNode::new(1);
        let r = TreeNode::new_left_right(4, 3, 6);
        let mut root = TreeNode::new(5);
        root.left = TreeNode::tree_node_wrap(l);
        root.right = TreeNode::tree_node_wrap(r);
        TreeNode::tree_node_wrap(root)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let target = Solution::tree_test_fixture_1();
        assert_eq!(Solution::is_valid_bst(target), true);
    }

    #[test]
    fn sample_2() {
        let target = Solution::tree_test_fixture_2();
        assert_eq!(Solution::is_valid_bst(target), false);
    }
}
