pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
    let (mut a, mut b, mut c) = (-1001i32, -1001i32, -1001i32);
    let (mut x, mut y) = (1001i32, 1001i32);

    for num in nums {
        let (pa, pb, px) = (a, b, x);
        a = a.max(num);
        b = b.max(pa.min(num));
        c = c.max(pb.min(num));
        x = x.min(num);
        y = y.min(px.max(num));
    }

    (a * b * c).max(a * x * y)
}
