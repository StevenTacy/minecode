pub fn distinct_subseq_ii(s: String) -> i32 {
    let mut seen = std::collections::HashSet::<&[u8]>::new();

    todo!()
}

fn dfs(s: &[u8], seen: &mut std::collections::HashSet<&[u8]>, i: usize, cur: &mut Vec<u8>) {
    if seen.contains(cur.as_slice()) {
        return;
    }

    cur.push(s[i]);
    dfs(s, seen, i + 1, cur);
    cur.pop();
}
