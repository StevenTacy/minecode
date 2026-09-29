// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        if head.is_none() {
            return vec![-1, -1];
        }

        let mut node_list = vec![];
        let mut cur = head.as_ref();

        while let Some(node) = cur {
            node_list.push(node.val);
            cur = node.next.as_ref();
        }

        let mut st: Vec<i32> = vec![];
        let mut min_dist = i32::MAX;
        for i in 1..node_list.len() - 1 {
            if (node_list[i] > node_list[i - 1] && node_list[i] > node_list[i + 1])
                || (node_list[i] < node_list[i - 1] && node_list[i] < node_list[i + 1])
            {
                if !st.is_empty() {
                    min_dist = min_dist.min(i as i32 - st[st.len() - 1]);
                }
                st.push(i as i32);
            }
        }

        if st.len() < 2 {
            vec![-1, -1]
        } else {
            vec![min_dist, st[st.len() - 1] - st[0]]
        }
    }

    pub fn nodes_between_critical_points_linked_list(head: Option<Box<ListNode>>) -> Vec<i32> {
        if head.is_none() {
            return vec![-1, -1];
        }

        let mut res = vec![-1, -1];
        let mut first_idx = 0;
        let mut prev_idx = 0;
        let mut cur_idx = 0;
        let mut min_dist = i32::MAX;
        let mut cur = head.as_ref();
        let mut cur_nxt = cur.unwrap().next.as_ref();

        while let Some(mut nxt) = cur_nxt {
            let nxt_nxt = nxt.next.as_ref();
            let mut cur = cur.unwrap();
            match nxt_nxt {
                Some(nxt_nxt) => {
                    if (nxt.val > cur.val && nxt.val > nxt_nxt.val)
                        || (nxt.val < cur.val && nxt.val < nxt_nxt.val)
                    {
                        if res[0] == -1 {
                            res[0] = cur_idx;
                        } else {
                            min_dist = min_dist.min(cur_idx - prev_idx);
                        }
                        prev_idx = cur_idx;
                    }
                }
                None => break,
            }
            cur_idx += 1;
            cur = nxt;
            cur_nxt = nxt_nxt;
        }
        todo!()
    }
}
