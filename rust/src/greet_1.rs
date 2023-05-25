// Make a function that will return a greeting statement that uses an input; your program should return, "Hello, <name> how are you doing today?".

// [Make sure you type the exact thing I wrote or the program may not execute properly]

pub fn greet(name: &str) -> String {
    let greeting = "Hello, ".to_string() + name + " how are you doing today?";
    return greeting.to_string();
}
