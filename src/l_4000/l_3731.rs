pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
    let max = *nums.iter().max().unwrap();
    let min = *nums.iter().min().unwrap();
    if (max - min + 1) as usize == nums.len() {
        return vec![];
    }

    let nums = nums.into_iter().collect::<std::collections::HashSet<_>>();
    let mut res = Vec::<i32>::with_capacity((max - min + 1) as usize - nums.len());
    for val in min..max {
        if nums.contains(&val) {
            continue;
        }
        res.push(val);
    }
    res
}
