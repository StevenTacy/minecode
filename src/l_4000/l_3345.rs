pub fn smallest_number(n: i32, t: i32) -> i32 {
    fn check(val: i32, t: i32) -> bool {
        let mut cur = 1;
        let mut x = val;
        while x > 0 {
            cur *= x % 10;
            x /= 10;
            if cur == 0 {
                break;
            }
        }
        cur % t == 0
    }

    let mut cur = n;
    while !check(cur, t) {
        cur += 1;
    }

    cur
}
