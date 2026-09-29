pub fn lex_palindromic_permutation(s: String, target: String) -> String {
    let n = s.len();
    if n == 1 {
        return if s > target { s } else { String::from("") };
    }

    let s = s.as_bytes();
    // let t = target.as_bytes();
    let mut cnt = [0; 26];
    for &c in s {
        cnt[(c - b'a') as usize] += 1;
    }

    // filter out if count of odd characters is more than 1
    let one_cnt = cnt.iter().filter(|&&c| c % 2 == 1).count();
    if one_cnt > 1 {
        return String::new();
    }

    // TODO determine the smallest lexicographical palindrome that is greater than target
    let mut res_arr = vec![0; s.len()];
    let mut cur = 0usize;
    for i in 0..26 {
        if cnt[i] % 2 == 1 {
            res_arr[s.len() / 2] = i as u8 + b'a';
            cnt[i] -= 1;
        }
        if cnt[i] > 0 {
            while cnt[i] > 0 {
                res_arr[cur] = i as u8 + b'a';
                res_arr[s.len() - 1 - cur] = i as u8 + b'a';
                cur += cnt[i] / 2;
            }
        }
    }

    String::from_utf8(res_arr).unwrap()
}
