struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.as_bytes();
        let mut res = Vec::new();
        let mut part = Vec::new();

        Self::dfs(s, 0, &mut part, &mut res);
        res
    }

    fn is_pali(s: &[u8], mut l: usize, mut r: usize) -> bool {
        while l < r {
            if s[l] != s[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }

    fn dfs(s: &[u8], i: usize, path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
        if i >= s.len() {
            res.push(path.clone());
            return;
        }

        for j in i..s.len() {
            if Self::is_pali(s, i, j) {
                path.push(String::from_utf8(s[i..=j].to_vec()).unwrap());
                Self::dfs(s, j + 1, path, res);
                path.pop();
            }
        }
    }
}
