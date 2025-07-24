pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    return (f - 32.0) / 1.8;
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    return (c * 1.8) + 32.0;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = fahrenheit_to_celsius(20.0);
//         assert_eq!(result, -6.666666666666666);
//     }
//     #[test]
//     fn it_workss() {
//         let result = celsius_to_fahrenheit(0.0);
//         assert_eq!(result, 32.0);
//     }
// }
