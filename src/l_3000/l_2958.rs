/// calculate the freq and pop when freq of specific num > k with left pointer
pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut freq = std::collections::HashMap::<i32, i32>::new();
    let mut left = 0usize;
    let mut res = 0i32;

    for (r, num) in nums.iter().enumerate() {
        *freq.entry(*num).or_insert(0) += 1;

        while *freq.get(num).unwrap() > k {
            freq.entry(nums[left]).and_modify(|v| *v -= 1);
            left += 1;
        }

        res = res.max((r - left) as i32 + 1);
    }

    res
}
