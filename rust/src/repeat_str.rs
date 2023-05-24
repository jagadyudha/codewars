// Write a function that accepts an integer n and a string s as parameters, and returns a string of s repeated exactly n times.

// Examples (input -> output)
// 6, "I"     -> "IIIIII"
// 5, "Hello" -> "HelloHelloHelloHelloHello"

pub fn repeat_str(src: &str, count: usize) -> String {
    let mut result: String = String::new();
    for _i in 0..count {
        result = result + src;
    }

    return result;
}
