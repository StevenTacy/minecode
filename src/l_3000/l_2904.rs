pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
    let s = s.as_bytes();
    let mut one_cnt = 0;
    let mut left = 0;
    let mut shortest = usize::MAX;
    let mut start = 0;

    for r in 0..s.len() {
        if s[r] == b'1' {
            one_cnt += 1;
        }

        if one_cnt == k {
            shortest = shortest.min(r - left + 1);
            start = left;
            println!("shortest: {}, start: {}", shortest, start);
        }

        while left < r && (one_cnt > k || s[left] == b'0') {
            one_cnt -= (s[left] - b'0') as i32;
            left += 1;
        }
    }

    if shortest == usize::MAX {
        return "".to_string();
    } else {
        String::from_utf8(s[start..start + shortest].to_vec()).unwrap_or("".to_string())
    }
}
