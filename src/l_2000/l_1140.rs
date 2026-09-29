pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let mut memo = vec![vec![0; piles.len()]; piles.len()];
    let mut suffix = piles
        .iter()
        .rev()
        .scan(0, |acc, &x| {
            println!("acc: {}, x: {}", acc, x);
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<_>>();
    max_stones(&mut memo, &mut suffix, 0, 1)
}

fn max_stones(memo: &mut Vec<Vec<i32>>, suffix: &mut [i32], cur_idx: usize, max_p: i32) -> i32 {
    // if opponent can take over remaining stones -> return the suffix directly
    if cur_idx + 2 * max_p as usize >= suffix.len() {
        return suffix[cur_idx];
    }

    // return memo result if existed
    if memo[cur_idx][max_p as usize] > 0 {
        return memo[cur_idx][max_p as usize];
    }

    let mut res = i32::MAX;
    for i in 1..=2 * max_p {
        res = res.min(max_stones(memo, suffix, cur_idx + i as usize, max_p.max(i)))
    }

    memo[cur_idx][max_p as usize] = suffix[cur_idx] - res;
    println!("res: {res}");
    println!("cur: {}", memo[cur_idx][max_p as usize]);
    memo[cur_idx][max_p as usize]
}
