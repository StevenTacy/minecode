pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let (min_i, _) = nums.iter().enumerate().min_by_key(|&(_, v)| v).unwrap();
    let (max_i, _) = nums.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();
    let left = min_i.min(max_i);
    let right = min_i.max(max_i);
    let res = (right + 1)
        .min(nums.len() - left)
        .min(left + 1 + nums.len() - right) as i32;
    res
}
