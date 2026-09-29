pub fn total_numbers(digits: Vec<i32>) -> i32 {
    let mut seen = [false; 1000];
    let n = digits.len();
    let mut res = 0;

    for i in 0..n {
        if digits[i] == 0 {
            continue;
        }
        for j in 0..n {
            if j == i {
                continue;
            }
            for k in 0..n {
                if k == i || k == j || digits[k] % 2 == 1 {
                    continue;
                }

                let cur = digits[i] * 100 + digits[j] * 10 + digits[k];
                if !seen[cur as usize] {
                    seen[cur as usize] = true;
                    res += 1;
                }
            }
        }
    }

    res
}
