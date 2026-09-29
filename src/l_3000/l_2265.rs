use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // global counts
    let mut cnt = 0;

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if node.is_none() {
            return (0, 0);
        }

        let node = node.as_ref().unwrap().borrow();
        let (l_sum, l_cnt) = dfs(node.left.clone());
        let (r_sum, r_cnt) = dfs(node.right.clone());

        if node.val == (l_sum + r_sum) / (l_cnt + r_cnt + 1) {
            // cnt += 1;
        }

        return (l_sum + r_sum + node.val, l_cnt + r_cnt + 1);
    }

    cnt
}
