/// 1. only if length of input is odd need to propagate due to even array we can always optimize to
///    win or tie the game
/// 2. recursion -> check either picking up front or back can yield the greater score of player 1
/// 3. memoization -> store the result of each subarray to avoid recomputation
pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if !n & 1 == 0 {
        return true;
    }

    let mut dp = vec![vec![-1; n]; n];
    dfs(0, n - 1, &nums, &mut dp) >= 0
}

fn dfs(i: usize, j: usize, nums: &[i32], dp: &mut Vec<Vec<i32>>) -> i32 {
    if dp[i][j] != -1 {
        return dp[i][j];
    }

    // the final middle value -> pick up for player 1
    if i == j {
        dp[i][j] = nums[i];
        return dp[i][j];
    }

    dp[i][j] = std::cmp::max(
        nums[i] - dfs(i + 1, j, nums, dp),
        nums[j] - dfs(i, j - 1, nums, dp),
    );

    dp[i][j]
}
