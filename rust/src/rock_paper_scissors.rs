// Let's play! You have to return which player won! In case of a draw return Draw!.

// Examples(Input1, Input2 --> Output):

// "scissors", "paper" --> "Player 1 won!"
// "scissors", "rock" --> "Player 2 won!"
// "paper", "paper" --> "Draw!"

pub fn rps(p1: &str, p2: &str) -> &'static str {
    let output: [&str; 3] = ["scissors", "rock", "paper"];

    fn find_index(output: [&str; 3], data: &str) -> usize {
        return output.iter().position(|&x| x == data).unwrap();
    }

    if find_index(output, p1) == 0 && find_index(output, p2) == 2 {
        return "Player 1 won!";
    } else if find_index(output, p1) == 2 && find_index(output, p2) == 0 {
        return "Player 2 won!";
    } else if find_index(output, p1) > find_index(output, p2) {
        return "Player 1 won!";
    } else if find_index(output, p1) < find_index(output, p2) {
        return "Player 2 won!";
    } else {
        return "Draw!";
    }
}
