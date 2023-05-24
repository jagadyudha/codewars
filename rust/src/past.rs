// Clock shows h hours, m minutes and s seconds after midnight.

// Your task is to write a function which returns the time since midnight in milliseconds.

// Example:
// h = 0 23 * 3600
// m = 1
// s = 1

// result = 61000
// Input constraints:

// 0 <= h <= 23
// 0 <= m <= 59
// 0 <= s <= 59

fn past(h: i32, m: i32, s: i32) -> i32 {
    (s * 1000) + (m * 60 * 1000) + (h * 3600 * 1000)
}
