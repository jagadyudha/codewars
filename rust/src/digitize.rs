// Convert number to reversed array of digits
// Given a random non-negative number, you have to return the digits of this number within an array in reverse order.

// Example(Input => Output):
// 35231 => [1,3,2,5,3]
// 0 => [0]

pub fn digitize(n: u64) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for i in n.to_string().chars() {
        let parse = i.to_string().parse::<u8>();
        match parse {
            Ok(value) => result.push(value),
            Err(error) => return [0].to_vec(),
        }
    }
    result.reverse();
    result
}
