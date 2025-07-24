pub fn sum(a: u8, b: u8) -> u8 {
    return a + b;
}

pub fn diff(a: i16, b: i16) -> i16 {
    return a - b;
}

pub fn pro(a: i8, b: i8) -> i8 {
    return a * b;
}

pub fn quo(a: f32, b: f32) -> f32 {
    return a / b;
}

pub fn rem(a: f32, b: f32) -> f32 {
    return a % b;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = quo(22.0, 2.0);
//         assert_eq!(result, 11.0);
//     }
//     #[test]
//     fn it_workss() {
//         let result = diff(-32768, 32766);
//         assert_eq!(result, 236);
//     }
// }
