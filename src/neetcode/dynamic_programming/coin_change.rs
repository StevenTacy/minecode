pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![amount + 1; amount + 1];

    for a in 1..=amount {
        for &c in coins.iter() {
            if c as usize <= a {
                dp[a] = dp[a].min(1 + dp[a - c as usize]);
            }
        }
    }

    if dp[amount] > amount {
        -1
    } else {
        dp[amount] as i32
    }
}
