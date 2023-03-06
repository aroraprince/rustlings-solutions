// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

fn main() {
    let question = 356789;
    let answer = square(question);
    println!("The square of {} is {}", question, answer);
}

fn square(num: i128) -> i128 {
    return num * num;
}
