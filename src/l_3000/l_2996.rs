pub fn missing_integer(nums: Vec<i32>) -> i32 {
    let seen: std::collections::HashSet<&i32> = nums.iter().collect();
    let mut sum = nums[0];
    for i in 1..=nums.len() {
        if nums[i] - 1 != nums[i - 1] {
            break;
        }
        sum += nums[i];
    }

    while seen.contains(&sum) {
        sum += 1;
    }

    sum
}
