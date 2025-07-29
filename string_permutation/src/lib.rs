use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut count_map = HashMap::new();

    for c in s1.chars() {
        *count_map.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        match count_map.get_mut(&c) {
            Some(count) => {
                if *count == 0 {
                    return false;
                }
                *count -= 1;
            }
            None => return false,
        }
    }

    true
}