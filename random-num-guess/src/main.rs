use rand::prelude::*;
use std::io;
use std::io::Write;
fn main() {
    let mut rng = rand::thread_rng();
    let y: u16 = rng.gen_range(1..100);
    print!("Please enter an integer: ");
    io::stdout().flush().unwrap(); // This will ensure that the prompt displays before waiting for input.

    while true {
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();
        let input: u16 = input_string
            .trim()
            .parse()
            .expect("Please enter a valid integer");
        if (input) > y {
            println!("Too high");
        } else if (input) < y {
            println!("Too low");
        } else {
            println!("That's correct guess");
            break;
        }
    }
}
