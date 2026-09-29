pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let div = nums.iter().sum::<i32>() - x;
    if div < 0 {
        return -1;
    }

    let (mut s, mut l) = (0, 0);
    let mut res = -1;
    for (i, n) in nums.iter().enumerate() {
        s += n;
        while s > div {
            s -= nums[l];
            l += 1;
        }

        if s == div {
            res = res.max((i - l + 1) as i32);
        }
    }

    if res == -1 {
        -1
    } else {
        nums.len() as i32 - res
    }
}
