pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();
    let shift = (key % 26) as i32;

    for c in input.chars() {
        if c.is_ascii_lowercase() {
            let base = 'a' as i32;
            let rotated = (c as i32 - base + shift + 26) % 26 + base;
            result.push(std::char::from_u32(rotated as u32).unwrap());
        } else if c.is_ascii_uppercase() {
            let base = 'A' as i32;
            let rotated = (c as i32 - base + shift + 26) % 26 + base;
            result.push(std::char::from_u32(rotated as u32).unwrap());
        } else {
            result.push(c);
        }
    }

    result
}
