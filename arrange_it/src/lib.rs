pub fn arrange_phrase(phrase: &str) -> String {
    let mut _temp: Vec<(i32, &str)> = Vec::new();

    for word in phrase.split_whitespace() {
        let mut number_str = String::new();
        for c in word.chars() {
            if c.is_numeric() {
                number_str.push(c);
            }
        }
        if !number_str.is_empty() {
            if let Ok(num) = number_str.parse::<i32>() {
                _temp.push((num, word));
            }
        }
    }
    _temp.sort_by_key(|k| k.0);
}
