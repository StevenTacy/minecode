pub fn lex_greater_permutation(s: String, target: String) -> String {
    let mut cnt = [0; 26];
    let s = s.as_bytes();
    let t = target.as_bytes();

    for (&cs, &ct) in s.iter().zip(t) {
        cnt[(cs - b'a') as usize] += 1;
        cnt[(ct - b'a') as usize] -= 1;
    }
    todo!()
}
