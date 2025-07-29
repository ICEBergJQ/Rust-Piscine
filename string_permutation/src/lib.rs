pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut char_count = [0; 256];

    for c in s1.chars() {
        char_count[c as usize] += 1;
    }

    for c in s2.chars() {
        char_count[c as usize] -= 1;
        if char_count[c as usize] < 0 {
            return false;
        }
    }

    true
}