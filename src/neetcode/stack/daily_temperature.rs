pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut st = Vec::<(i32, usize)>::with_capacity(temperatures.len());
    let mut res = vec![0; temperatures.len()];
    for (i, t) in temperatures.iter().enumerate() {
        while let Some(&(last_t, last_i)) = st.last() {
            if last_t < *t {
                res[last_i] = (i - last_i) as i32;
                st.pop();
            } else {
                break;
            }
        }
        st.push((*t, i));
    }
    res
}
