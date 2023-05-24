mod find_max_min;

fn main() {
    let data: [i32; 8] = [4, 6, 2, 1, 9, 63, -134, 566];
    print!("max = {}", find_max_min::maximum(&data))
}
