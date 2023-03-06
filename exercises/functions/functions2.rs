// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me(10);
}

fn call_me(num:i8) {
    for i in num-6..num {
        println!("Ring! Call number {}", i + 1);
    }
}
