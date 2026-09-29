/// manage left and right index -> left index holds up starting point for current substring
/// if one char occur >= 1 -> move left pointer to right til there is no duplicate char in the
/// substringj
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut seen = std::collections::HashMap::<u8, usize>::new();
    let mut l = 0;
    let mut res = 0;
    let s = s.as_bytes();

    for (i, c) in s.iter().enumerate() {
        if let Some(&idx) = seen.get(c) {
            l = l.max(idx + 1);
        }

        seen.insert(*c, i);
        res = res.max(i - l + 1);
    }

    res as i32
}
