/// check either we use current one and jump to i + 2 or we skip current one and jump to i + 1
/// top-down approach
pub fn rob_bottom_up(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0i32;
    }
    if nums.len() == 1 {
        return nums[0];
    }
    let first = helper(&nums[..nums.len() - 1]);
    let last = helper(&nums[1..]);
    println!("first: {}, last: {}", first, last);
    // helper(&nums[1..]).max(helper(&nums[..nums.len() - 1]))
    first.max(last)
}

fn helper(nums: &[i32]) -> i32 {
    println!("nums: {:?}", nums);
    let mut dp = vec![0; nums.len() + 2];

    for (i, num) in nums.iter().enumerate().rev() {
        dp[i] = dp[i + 1].max(num + dp[i + 2]);
    }

    dp[0]
}
