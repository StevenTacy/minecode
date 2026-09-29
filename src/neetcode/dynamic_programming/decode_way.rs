pub fn num_decodings(s: String) -> i32 {
    let mut seen = std::collections::HashMap::<usize, i32>::new();
    let s = s.as_bytes();
    seen.insert(s.len(), 1);
    dfs(s, 0, &mut seen)
}

pub fn num_decodings_bottom_up(s: String) -> i32 {
    let s = s.as_bytes();
    let mut dp = vec![0; s.len()];
    todo!()
}

fn dfs(s: &[u8], i: usize, seen: &mut std::collections::HashMap<usize, i32>) -> i32 {
    if i > s.len() {
        return 0;
    }
    if s[i] == b'0' {
        return 0;
    }
    if let Some(&val) = seen.get(&i) {
        return val;
    }

    let mut res = dfs(s, i + 1, seen);
    if i + 1 < s.len() && (s[i] == b'1' || (s[i] == b'2' && s[i + 1] <= b'6')) {
        res += dfs(s, i + 2, seen);
    }

    seen.insert(i, res);
    res
}
