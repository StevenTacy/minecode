pub fn combination_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    nums.sort();

    fn dfs(
        nums: &[i32],
        target: i32,
        idx: usize,
        cur: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        cur_sum: i32,
    ) {
        if cur_sum + nums[idx] == target {
            res.push(cur.clone());
            return;
        }

        for j in idx..nums.len() {
            if cur_sum + nums[j] > target {
                return;
            }
            cur.push(nums[j]);
            dfs(nums, target, j, cur, res, cur_sum + nums[j]);
            cur.pop();
        }
    }

    dfs(&nums, target, 0, &mut vec![], &mut res, 0);
    res
}
