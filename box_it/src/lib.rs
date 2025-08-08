pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut result = Vec::new();

    for token in s.split_whitespace() {
        let is_k = token.ends_with('k') || token.ends_with('K');
        let num_str = if is_k {
            &token[..token.len() - 1]
        } else {
            token
        };

        if let Ok(n) = num_str.parse::<f32>() {
            let value = if is_k {
                (n * 1000.0).round() as u32
            } else {
                n.round() as u32
            };
            result.push(Box::new(value));
        }
    }

    result
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut result = Vec::new();

    for boxed in a {
        result.push(*boxed);
    }

    result
}
