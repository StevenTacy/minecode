/// use a 2-D array to count number of overlaps using differences between two points
pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
    let ones1: Vec<(i32, i32)> = img1
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &val)| {
                if val == 1 {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    let ones2: Vec<(i32, i32)> = img2
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &val)| {
                if val == 1 {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    let total_len = img1.len() + img2.len();
    let mut cnt = vec![vec![0; total_len]; total_len];
    let mut best = 0;

    for &(x1, y1) in &ones1 {
        for &(x2, y2) in &ones2 {
            let dx = x1 - x2 + img1.len() as i32;
            let dy = y1 - y2 + img1.len() as i32;
            cnt[dx as usize][dy as usize] += 1;
            best = best.max(cnt[dx as usize][dy as usize]);
        }
    }

    best
}
