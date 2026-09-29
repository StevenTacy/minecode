pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let mut heap = std::collections::BinaryHeap::<(i32, i32, i32)>::with_capacity(k);

    for (x, y) in points.iter().map(|p| (p[0], p[1])) {
        let dis = x * x + y * y;
        heap.push((dis, x, y));
        if heap.len() > k {
            heap.pop();
        }
    }

    heap.into_iter().map(|(_, x, y)| vec![x, y]).collect()
}
