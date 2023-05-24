pub fn maximum(arr: &[i32]) -> i32 {
    let maxValue: Option<&i32> = arr.iter().max();
    match maxValue {
        Some(max) => *max,
        None => 0,
    }
}

pub fn minimum(arr: &[i32]) -> i32 {
    let minimumValue: Option<&i32> = arr.iter().min();
    match minimumValue {
        Some(min) => *min,
        None => 0,
    }
}
