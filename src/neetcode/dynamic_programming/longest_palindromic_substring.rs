/// use current char as center and expand to left and right
/// be aware of handle even and odd length palindrome
pub fn longest_palindrome(s: String) -> String {
    let s = s.as_bytes();
    // start index for match palindrome
    let mut res_idx = 0usize;
    let mut res_len = 0usize;

    for i in 0..s.len() {
        let (mut l, mut r) = (i as isize, i as isize);
        // odd handling
        while l >= 0 && (r as usize) < s.len() && s[l as usize] == s[r as usize] {
            if (r - l + 1) as usize > res_len {
                res_idx = l as usize;
                res_len = (r - l + 1) as usize;
            }
            l -= 1;
            r += 1;
        }

        let (mut l, mut r) = (i as isize, (i + 1) as isize);
        while l >= 0 && (r as usize) < s.len() && s[l as usize] == s[r as usize] {
            if (r - l + 1) as usize > res_len {
                res_idx = l as usize;
                res_len = (r - l + 1) as usize;
            }
            l -= 1;
            r += 1;
        }
    }

    String::from_utf8(s[res_idx..res_idx + res_len].to_vec()).unwrap()
}
