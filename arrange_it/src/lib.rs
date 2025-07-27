pub fn arrange_phrase(phrase: &str) -> String {
    let mut _temp: Vec<(i32, String)> = Vec::new();
    let mut res = String::new();

    for word in phrase.split_whitespace() {
        let mut _wrd = String::new();
        let mut number_str = String::new();

        for c in word.chars() {
            if c.is_numeric() {
                number_str.push(c);
            } else {
                _wrd.push(c);
            }
        }

        if let Ok(num) = number_str.parse::<i32>() {
            _temp.push((num, _wrd));
        }
    }

    _temp.sort_by_key(|k| k.0);

    for (_, wrd) in &_temp {
        res.push_str(wrd);
        res.push(' ');
    }
    res.pop();
    res
}