pub fn smallest_palindrome(s: String) -> String {
    let (n, mut j) = (s.len(), 0);
    let mut freq = [0i32; 26];

    for c in s.as_bytes().iter().take(n / 2) {
        freq[(*c - b'a') as usize] += 1;
    }

    let mut res_vec = Vec::<u8>::with_capacity(n);
    for i in 0..26 {
        while freq[i] > 0 {
            let char = b'a' + i as u8;
            println!("char");
            res_vec[j] = char;
            res_vec[n - 1 - j] = char;
            j += 1;
            freq[i] -= 1;
        }
    }

    res_vec.into_iter().map(|n| n as char).collect()
}
