pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = std::collections::BinaryHeap::from(stones);

    while heap.len() > 1 {
        let first = heap.pop().unwrap();
        let sec = heap.pop().unwrap();

        if first > sec {
            heap.push(first - sec);
        } else if first < sec {
            heap.push(sec - first);
        }
    }

    heap.pop().unwrap_or(0)
}
