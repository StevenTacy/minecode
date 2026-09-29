use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

struct Solution;

/// binary tree node number should be odd. root for 1 and the other either have 0 or 2 child
/// Use this method to iterate every valid pair of left and right (which two number should be odd)
/// cuz they become subroot (which root should be 1 and the other either have 0 or 2 child)
/// then iterate through such combinations
impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n % 2 == 0 {
            return vec![];
        }

        if n == 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))];
        }

        let mut res: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

        // iterate through all possible old combination of left and right
        for i in (1..n).step_by(2) {
            let left = Self::all_possible_fbt(i);
            let right = Self::all_possible_fbt(n - i - 1);

            for l in &left {
                for r in &right {
                    let mut root = TreeNode::new(0);
                    root.left = l.clone();
                    root.right = r.clone();
                    res.push(Some(Rc::new(RefCell::new(root))));
                }
            }
        }

        res
    }
}
