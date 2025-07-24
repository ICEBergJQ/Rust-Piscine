pub fn insert(vec: &mut Vec<String>, val: String) {
}

pub fn at_index(slice: &[String], index: usize) -> &str {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
