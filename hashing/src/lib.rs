use std::collections::HashMap;
pub fn mean(list: &[i32]) -> f64 {
    if list.is_empty() {
        return 0.0; 
    }
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    if list.is_empty() {
        return 0;
    }
    let mut sorted_list = list.to_vec();
    sorted_list.sort_unstable();
    let mid = sorted_list.len() / 2;
    if sorted_list.len() % 2 == 0 {
        (sorted_list[mid - 1] + sorted_list[mid]) / 2
    } else {
        sorted_list[mid]
    }
}

pub fn mode(list: &[i32]) -> i32 {

    if list.is_empty() {
        return 0;
    }

    let mut frequency_map = HashMap::new();
    for &value in list {
        *frequency_map.entry(value).or_insert(0) += 1;
    }

    frequency_map.into_iter().max_by_key(|&(_, count)| count).map_or(0, |(value, _)| value)
}