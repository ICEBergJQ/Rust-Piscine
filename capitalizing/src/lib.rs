pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let mut chars = input.chars();
    let first_char = chars.next().unwrap().to_uppercase().to_string();
    let rest: String = chars.collect();
    first_char + &rest
}

pub fn title_case(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c.to_lowercase().next().unwrap());
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let mut result = String::new();

    for c in input.chars() {
        if c.is_uppercase() {
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c.to_uppercase().next().unwrap());
        }
    }
    result
}