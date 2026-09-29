pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    let n = arr.len();
    let (mut s, mut ans, mut l) = (0i32, n as i32 + 1, 0usize);
    let mut dp = vec![n as i32; n + 1];

    for r in 0..n {
        s += arr[r];
        while s > target {
            s -= arr[l];
            l += 1;
        }

        dp[r + 1] = dp[r];
        if s == target {
            let len = r - l + 1;
            ans = ans.min(len as i32 + dp[l]);
            dp[r + 1] = dp[r].min(len as i32);
        }
        println!("cur_val: {}, ans: {ans}", dp[r + 1]);
    }

    if ans > n as i32 { -1 } else { ans }
}
