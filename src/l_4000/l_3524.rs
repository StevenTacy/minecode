pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
    let n = nums.len();
    let k = k as usize;
    let mut dp = vec![0i64; k];
    let mut res = vec![0i64; k];

    for i in 0..n {
        let mut temp = vec![0i64; k];
        temp[nums[i] as usize % k] += 1;

        for j in 0..k {
            temp[((nums[i] as i64 * j as i64) % k as i64) as usize] += dp[j];
        }

        dp = temp;
        for r in 0..k {
            res[r] += dp[r];
        }
    }

    res
}
