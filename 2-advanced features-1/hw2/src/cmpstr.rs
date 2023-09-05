pub fn compare_string(x: &str, y: &str) -> bool {
    let len_x = x.len();
    let len_y = y.len();
    let min_len = if len_x > len_y { len_y } else { len_x};
    for i in 0..min_len {
        if x.as_bytes()[i] > y.as_bytes()[i] {
            return true;
        }
    }
    false
}