pub fn longest_repeating(s: String, query_characters: String, query_indices: Vec<i32>) -> Vec<i32> {
    let s = s.as_bytes();
    let q = query_characters.as_bytes();
    let mut max_counts = 0;
    let mut cur_counts = 1;
    let mut freq = [0i32; 26];
    freq[s[0] as usize - b'a' as usize] += 1;

    for i in 0..s.len() - 1 {
        if s[i] == s[i + 1] {
            cur_counts += 1;
        } else {
            max_counts = max_counts.max(cur_counts);
            cur_counts = 1;
        }
        freq[(s[i] - b'a') as usize] += 1;
    }

    println!("max_counts: {}, freq: {:?}", max_counts, freq);

    let mut res = vec![max_counts; query_indices.len()];
    for (j, (&c, &i)) in q.iter().zip(query_indices.iter()).enumerate() {
        let c_idx = (c - b'a') as usize;
        if freq[c_idx] + 1 < max_counts {
            continue;
        }

        let mut r = 0;
        while (i as usize + r) < s.len() && s[i as usize + r] == c {
            r += 1;
        }

        let mut l = 0;
        while (i as usize >= l) && s[i as usize - l] == c {
            l += 1;
        }

        res[j] = res[j].max((l + r + 1) as i32);
    }

    res
}
