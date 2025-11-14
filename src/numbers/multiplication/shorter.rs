pub fn get_shorter(x: Vec<u8>, y: Vec<u8>) -> (Vec<u8>, usize) {
    let x_len = x.len();
    let y_len = y.len();
    if x_len < y_len {
        (x, y_len - x_len)
    } else {
        (y, x_len - y_len)
    }
}