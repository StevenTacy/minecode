/// check either we use current one and jump to i + 2 or we skip current one and jump to i + 1
/// top-down approach
pub fn rob(nums: Vec<i32>) -> i32 {
    let mut memo = vec![-1; nums.len()];

    fn dfs(nums: &[i32], memo: &mut [i32], idx: usize) -> i32 {
        if idx >= nums.len() {
            return 0;
        }
        if memo[idx] != -1 {
            return memo[idx];
        }
        memo[idx] = dfs(nums, memo, idx + 1).max(nums[idx] + dfs(nums, memo, idx + 2));
        memo[idx]
    }

    dfs(&nums, &mut memo, 0)
}

pub fn rob_bottom_up(nums: Vec<i32>) -> i32 {
    // if nums.is_empty() {
    //     return 0i32;
    // }
    // if nums.len() == 1 {
    //     return nums[0];
    // }

    let mut dp = vec![0; nums.len() + 2];

    for (i, num) in nums.iter().enumerate().rev() {
        dp[i] = dp[i + 1].max(num + dp[i + 2]);
    }

    dp[0]
}
