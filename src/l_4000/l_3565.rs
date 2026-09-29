pub fn max_product(n: i32) -> i32 {
    let mut n = n;
    let (mut first, mut sec) = (0, 0);

    while n > 0 {
        let cur = n % 10;
        if cur > first {
            sec = first;
            first = cur;
        } else if cur > sec {
            sec = cur;
        }
        n /= 10;
    }

    first * sec
}
