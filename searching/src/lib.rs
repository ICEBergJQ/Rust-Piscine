pub fn search(ar: &[i32], key: i32) -> Option<usize> {
    for (i, &v) in ar.iter().enumerate().rev() {
        if v == key {
            return Some(i);
        }
    }
    None
}
