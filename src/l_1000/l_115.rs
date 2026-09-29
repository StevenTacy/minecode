pub fn num_distinct(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let (m, n) = (s.len(), t.len());
    if m < n {
        return 0;
    }

    let mut dp = vec![0; s.len() + 1];
    dp[n] = 1;

    for i in (0..m).rev() {
        for j in 0..n {
            if s[i] == t[j] {
                dp[j] += dp[j + 1];
            }
        }
    }

    dp[0]
}
