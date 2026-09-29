pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
    let (n, m) = (word1.len(), word2.len());
    let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
    let mut j = m as i32 - 1;
    let mut suffix = vec![-1; m];

    for i in (0..n).rev() {
        if j >= 0 && word1[i] == word2[j as usize] {
            suffix[j as usize] = i as i32;
            j -= 1;
        }
    }

    let mut res = vec![];
    let mut j = 0;
    let mut skip = 0;
    for i in 0..n {
        if j == m {
            break;
        }
        // determine if skip == 0 (replacement not used yet) and next char is behind current
        // position then we continue
        if word1[i] == word2[j] || (skip == 0 && (j == m - 1 || suffix[j + 1] > i as i32)) {
            if word1[i] != word2[j] {
                skip += 1;
            }
            res.push(i as i32);
            j += 1;
        }
    }

    if j == m { res } else { vec![] }
}
