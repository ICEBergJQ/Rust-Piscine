pub fn delete_and_backspace(s: &mut String) {
    let mut op_c: usize = 0;
    let mut temp = String::new();
    for c in s.chars() {
        if c == '-' {
            temp.pop();
            continue;
        } else if c == '+' {
            op_c += 1;
            continue;
        }
        if op_c > 0 {
            op_c -= 1;
            continue;
        } else {
            temp.push(c);
        }
    }
    *s = temp;
}

pub fn do_operations(v: &mut [String]) {
    let mut res: Vec<String> = Vec::new();
    let mut op: char = '.';
    for e in &mut *v {
        for c in e.chars() {
            if c == '+' || c == '-' {
                op = c;
                break;
            }
        }
        let mut spl = e.split(op);
        let lhs = spl.next().unwrap().parse::<i32>().unwrap();
        let rhs = spl.next().unwrap().parse::<i32>().unwrap();
        let result = match op {
            '+' => lhs + rhs,
            '-' => lhs - rhs,
            _ => unreachable!(),
        };
        res.push((result).to_string());
    }

    for (i, val) in res.into_iter().enumerate() {
        v[i] = val;
    }
}
