pub fn reverse_degree(s: String) -> i32 {
    let s = s.as_bytes();
    let mut res = 0;

    for (i, c) in s.iter().enumerate() {
        res += (26 - (*c - b'a') as i32) * (i as i32 + 1);
    }

    res
}
