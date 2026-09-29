pub fn check_divisibility(n: i32) -> bool {
    let mut sum = 0;
    let mut product = 1;
    let mut mut_n = n;

    while mut_n > 0 {
        sum += mut_n % 10;
        product *= mut_n % 10;
        mut_n /= 10;
    }

    n % (sum + product) == 0
}
