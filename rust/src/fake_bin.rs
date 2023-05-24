// Given a string of digits, you should replace any digit below 5 with '0' and any digit 5 and above with '1'. Return the resulting string.

// Note: input will never be an empty string

pub fn fake_bin(s: &str) -> String {
    fn checker(val: i32) -> i32 {
        match val {
            val if val >= 5 => 1,
            _ => 0,
        }
    }

    let mut result: Vec<String> = Vec::new();

    for i in s.chars() {
        let parsed = i.to_string().parse::<i32>();
        match parsed {
            Ok(value) => result.push(checker(value).to_string()),
            Err(error) => return error.to_string(),
        }
    }
    result.join("")
}
