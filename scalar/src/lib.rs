pub fn sum(a: u8, b: u8) -> u8 {
    return a + b;
}

pub fn diff(a: i16, b: i16) -> i16 {
    return a - b;
}

pub fn pro(a: i8, b: i8) -> i8 {
    return a * b;
}

pub fn quo(a: i32, b: i32) -> i32 {
    return a / b;
}

pub fn rem(a: i32, b: i32) -> i32 {
    return a % b;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = diff(234, 2);
//         assert_eq!(result, 232);
//     }
//     #[test]
//     fn it_workss() {
//         let result = diff(-32768, 32766);
//         assert_eq!(result, 236);
//     }
// }
