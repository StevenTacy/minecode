pub fn smallest_subsequence(s: String) -> String {
    let s = s.as_bytes();
    let mut freq = [0u32; 26];
    s.iter().for_each(|c| {
        freq[(*c - b'a') as usize] += 1;
    });

    let mut st = vec![];
    let mut seen = [0u32; 26];

    for c in s {
        let idx = (*c - b'a') as usize;
        freq[idx] -= 1;
        if seen[idx] > 0 {
            continue;
        }
        while !st.is_empty()
            && *st.last().unwrap() > *c
            && freq[(*st.last().unwrap() - b'a') as usize] > 0
        {
            let character = st.pop().unwrap();
            seen[(character - b'a') as usize] -= 1;
        }
        st.push(*c);
        seen[idx] += 1;
    }

    String::from_utf8_lossy(&st).to_string()
}
