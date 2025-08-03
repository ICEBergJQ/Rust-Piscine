pub fn stars(n: u32) -> String {
    let mut result = String::new();  
    for i in 0..2_usize.pow(n) {
        result.push('*');
    }
    result
}