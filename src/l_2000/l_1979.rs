pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let max_val = *nums.iter().max().unwrap_or(&0);
    let min_val = *nums.iter().min().unwrap_or(&0);
    gcd(max_val, min_val)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}
