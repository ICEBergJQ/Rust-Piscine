pub fn to_url(s: &str) -> String {
    let mut res = String::new();

    for i in s.chars() {
        if i == ' '{
            for u in "%20".chars() {
                res.push(u)    
            }
        } else {
            res.push(i)
        }
    }
    res
}