pub fn sum_game(num: String) -> bool {
    let num = num.as_bytes();
    let get = |num: &[u8]| -> (i32, i32) {
        let mut q = 0;
        let mut sum = 0;
        for &c in num {
            if c == b'?' {
                q += 1;
            } else {
                sum += (c - b'0') as i32;
            }
        }
        (q, sum)
    };
    let (q_l, sum_l) = get(&num[..num.len() / 2]);
    let (q_r, sum_r) = get(&num[num.len() / 2..]);

    // alice can win if question marks are odd
    ((q_l + q_r) % 2 == 1) || (sum_l - sum_r) != (q_r - q_l) * 9 / 2
}
