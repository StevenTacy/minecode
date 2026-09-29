use std::i32;

pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }
    let mut max_vec = vec![0; nums.len()];
    let mut min_vec = vec![i32::MAX; nums.len()];
    max_vec[0] = nums[0];
    min_vec[nums.len() - 1] = nums[nums.len() - 1];

    for i in 1..nums.len() {
        max_vec[i] = nums[i].max(max_vec[i - 1]);
    }

    for i in (0..nums.len() - 1).rev() {
        min_vec[i] = nums[i].min(min_vec[i + 1]);
    }

    for (i, (&maxv, &minv)) in max_vec.iter().zip(min_vec.iter()).enumerate() {
        if maxv - minv <= k {
            return i as i32;
        }
    }

    -1
}
