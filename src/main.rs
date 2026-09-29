fn main() {
    let open = '(' as u8;
    let close = ')' as u8;
    let val = 0b0000_1111u8;
    println!("open: {}", open);
    println!("close: {}", close);
    println!("shift: {}", (open & 1) << 1);
    println!("val: {:b}", !val);
}
