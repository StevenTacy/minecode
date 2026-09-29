pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
    let (m, n) = (grid.len(), grid[0].len());

    // path length should modifiable by 2
    if (m + n - 1) % 2 != 0 {
        return false;
    }

    // first and last cell should be '(' and ')'
    if grid[0][0] != '(' || grid[m - 1][n - 1] != ')' {
        return false;
    }

    let mut dp = vec![vec![vec![false; m + n - 1]; n]; m];
    dp[0][0][1] = true;

    for i in 0..m {
        for j in 0..n {
            let cur_bracket = if grid[i][j] == '(' { 1 } else { -1 };
            if i > 0 {
                for k in 0..m + n - 1 {
                    if !dp[i - 1][j][k] {
                        continue;
                    }

                    let next = k as isize + cur_bracket;

                    if next >= 0 {
                        dp[i][j][next as usize] = true;
                    }
                }
            }

            if j > 0 {
                for k in 0..m + n - 1 {
                    if !dp[i][j - 1][k] {
                        continue;
                    }

                    let next = k as isize + cur_bracket;

                    if next >= 0 {
                        dp[i][j][next as usize] = true;
                    }
                }
            }
        }
    }

    dp[m - 1][n - 1][0]
}

pub fn hash_has_valid_path(grid: Vec<Vec<char>>) -> bool {
    use std::collections::{HashMap, HashSet};
    let (m, n) = (grid.len(), grid[0].len());
    if !(m + n) & 1 != 0 || grid[0][0] != '(' || grid[m - 1][n - 1] != ')' {
        return false;
    }

    let mut path_map = HashMap::<(usize, usize), HashSet<i32>>::new();
    path_map.insert((0, 0), HashSet::from([0]));

    for i in 0..m {
        for j in 0..n {
            let val = 1 - ((grid[i][j] as u8 & 1) << 1) as i32;

            let cur_entry = path_map.entry((i, j)); //.or_insert(HashSet::new());
            let cur_entry = path_map.get(&(i, j));
            if let Some(cur_entry) = cur_entry {
                for &k in cur_entry.iter() {
                    let nxt = k + val;
                    if nxt >= 0 {}
                }
            } else {
            }
        }
    }
    todo!()
}
