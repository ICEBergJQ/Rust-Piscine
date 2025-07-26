pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let c_f64 = c as f64;
    (c, c_f64.exp(), c_f64.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut expo = String::new();

    for v in a.split_whitespace() {
        expo.push_str(
            &v.parse::<f64>()
                .ok()
                .expect("error parsing v to f64")
                .exp()
                .to_string(),
        );
        expo.push(' ');
    }
    (a, expo.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res: Vec<f64> = vec![];
    for v in &b {
        let f = *v as f64;
        res.push(f.abs().ln())
    }
    (b, res)
}
