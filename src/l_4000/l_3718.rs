/// using an fixed size array to store the result num / k as index
/// iterate through array to find the first missing index which value is 0
/// if iterate all over we do not see any missing index -> return 101 (biggest index)
pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
    const LARGEST: usize = 101;
    let mut seen = [0u8; 101];
    for num in nums {
        if num % k != 0 {
            continue;
        }
        seen[(num / k) as usize] = 1;
    }

    for i in 1..seen.len() {
        if seen[i] == 0 {
            return i as i32 * k;
        }
    }

    0i32
}
