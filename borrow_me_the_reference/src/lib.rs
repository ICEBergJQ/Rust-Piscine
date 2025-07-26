pub fn delete_and_backspace(s: &mut String) {
    let mut back_s: bool = false;
    let mut op_c: usize = 0;
    let mut temp = String::new();
    let clone = s.clone();
    let mut next_ind: usize = 0;
    for (i, c) in clone.char_indices() {
        if i < next_ind {
            continue;
        }
        if c == '-' || c == '+' {
            op_c = op_count(i, &clone, c);
            next_ind = i + op_c;
            if c == '-' {
                back_s = true;
            } else {
                next_ind += op_c;
            }
        } else {
            temp.push(c);
        }
        if back_s {
            temp.truncate(temp.len() - op_c);
            back_s = false;
        }
    }
    *s = temp;
}

fn op_count(ind: usize, s: &str, op: char) -> usize {
    let mut count = 0;
    for c in s[ind..].chars() {
        if c == op {
            count += 1;
        } else {
            break;
        }
    }

    count
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
        res.push((lhs + rhs).to_string());
    }

    for (i, val) in res.into_iter().enumerate() {
        v[i] = val;
    }
}
