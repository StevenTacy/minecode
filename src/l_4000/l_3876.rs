/// intuition:
/// 1. since each number is unique -> we can determine that
/// nums2 will be odd or even based on smallest number
/// due to we have limitation that nums[i] - nums[j] >= 1
pub fn uniform_array(nums1: Vec<i32>) -> bool {
    let mut min_val = i32::MAX;
    let mut is_odd = false;
    for &num in &nums1 {
        if num < min_val {
            min_val = num;
        }

        if num % 2 == 1 {
            is_odd = true;
        }
    }

    if min_val % 2 == 1 { true } else { !is_odd }
}
