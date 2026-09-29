/// sliding window
/// poping from the left if current iter index have more than 2 occurrence
pub fn maximum_length_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let mut l = 0usize;
    let mut res = 0i32;
    let mut freq = [0i32; 26];

    for (r, &c) in s.iter().enumerate() {
        freq[(c - b'a') as usize] += 1;
        while freq[(c - b'a') as usize] > 2 {
            freq[(s[l] - b'a') as usize] -= 1;
            l += 1;
        }
        res = res.max((r - l + 1) as i32);
    }

    res
}
