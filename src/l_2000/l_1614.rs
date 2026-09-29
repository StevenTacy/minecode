pub fn max_depth(s: String) -> i32 {
    let mut cur_depth = 0;
    let mut res = 0;
    for c in s.chars() {
        match c {
            '(' => cur_depth += 1,
            ')' => {
                res = res.max(cur_depth);
                cur_depth -= 1;
            }
            _ => {}
        }
    }

    res
}
