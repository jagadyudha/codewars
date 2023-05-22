pub fn maps(values: &Vec<i32>) -> Vec<i32> {
    let result: Vec<i32> = values.into_iter().map(|f: &i32| f * 2).collect();
    return result;
}
