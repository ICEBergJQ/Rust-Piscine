pub fn is_pangram(s: &str) -> bool {
    let mut letters = [false; 26];

    for c in s.to_ascii_lowercase().chars() {
        if ('a'..='z').contains(&c) {
            letters[(c as u8 - b'a') as usize] = true;
        }
    }

    letters.iter().all(|&present| present)
}
