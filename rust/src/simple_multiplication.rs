// This kata is about multiplying a given number by eight if it is an even number and by nine otherwise.

pub fn simple_multiplication(number: u8) -> u8 {
    match number {
        number if number % 2 == 0 => number * 8,
        _ => number * 9,
    }
}
