pub fn number_logic(num: u32) -> bool {
    let digits_str = num.to_string();
    let n = digits_str.len() as u32;

    let sum: u32 = digits_str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(n))
        .sum();

    sum == num
}
