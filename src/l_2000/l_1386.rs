/// bitwise comparison problem
/// 1. each row at max we can have two combinations -> (2, 3, 4, 5) and (6, 7, 8, 9)
///    we cannot overlap (4, 5, 6, 7)
/// 2. utilize three primitive -> left, middle, and right to compare the reserved seats occupied the
///    left, middle, and right combinations and use HashMap to store the row number as key and
///    bitwise repr as value
/// 3. use ( n - HashMap.len() ) * 2 -> which we preeliminate the rows seat from 2 - 9 that been
///    occupied, then we iterate through each row to see if its own seat match either left, or
///    middle, or right combination, if so we add 1 to the total count
pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
    let left = 0b11110000;
    let middle = 0b00111100;
    let right = 0b00001111;
    let mut occupied = std::collections::HashMap::<i32, i32>::new();

    for seat in reserved_seats {
        let row = seat[0];
        let cell = seat[1];
        if cell >= 2 && cell <= 9 {
            *occupied.entry(row).or_insert(0) |= 1 << (cell - 2);
        }
    }

    let mut ans = (n - occupied.len() as i32) * 2;
    for val in occupied.values() {
        if (val | left) == left || (val | middle) == middle || (val & right) == right {
            ans += 1;
        }
    }

    ans
}
