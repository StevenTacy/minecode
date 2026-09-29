pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
    use std::collections::{HashMap, HashSet};
    let classroom = classroom
        .into_iter()
        .map(|c| c.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut l_cnt = 0;

    todo!()
}
