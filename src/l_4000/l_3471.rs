pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    if k == n as i32 {
        return *nums.iter().max().unwrap();
    }

    let mut cnt = [0; 51];
    for &x in &nums {
        cnt[x as usize] += 1;
    }

    if k == 1 {
        for val in (0..=50).rev() {
            if cnt[val as usize] == 1 {
                return val;
            }
        }
        return -1;
    }

    let mut res = -1;
    if cnt[nums[0] as usize] == 1 {
        res = res.max(nums[0]);
    }
    if cnt[nums[n - 1] as usize] == 1 {
        res = res.max(nums[n - 1]);
    }

    res
}
