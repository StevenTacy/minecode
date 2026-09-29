use std::clone;

pub fn reverse_parentheses(s: String) -> String {
    let n = s.len();
    let s = s.as_bytes();

    let mut open_idx: Vec<usize> = Vec::with_capacity(n / 2);
    let mut pair_cnts: Vec<usize> = vec![0; n];
    let mut pairs = 0;

    for (i, &c) in s.iter().enumerate() {
        match c {
            b'(' => {
                open_idx.push(i);
                pairs += 1;
            }
            b')' => {
                let open = open_idx.pop().unwrap();
                pair_cnts[i] = open;
                pair_cnts[open] = i;
            }
            _ => continue,
        }
    }

    let mut res: Vec<u8> = Vec::with_capacity(n - pairs * 2);
    let mut cur_idx = 0;
    let mut dir = 1;

    while cur_idx < n {
        match s[cur_idx] {
            b'(' | b')' => {
                cur_idx = pair_cnts[cur_idx];
                dir *= -1;
            }
            _ => res.push(s[cur_idx]),
        }
        cur_idx = (cur_idx as isize + dir) as usize;
    }

    String::from_utf8(res).unwrap()
}
