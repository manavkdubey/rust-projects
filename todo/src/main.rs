use std::fmt::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
pub fn read_file() -> io::Result<()> {
    let contents = fs::read_to_string("todo.txt")?;
    println!("{}", contents);
    Ok(())
}
pub fn write_file_new(contents: &str) -> io::Result<()> {
    fs::write("todo.txt", contents)
}
pub fn append_to_file(contents: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true) // If the file doesn't exist, create it
        .append(true) // Open file in append mode
        .open("todo.txt")?;

    writeln!(file, "{}", contents) // writes the content followed by a newline
}

fn main() {
    loop {
        println!("Enter a choice :\n1. Enter a new list\n2.Append previous list\n3.Display list");
        let mut choice_str = String::new();
        io::stdin()
            .read_line(&mut choice_str)
            .expect("Failed to read line");
        let choice: i32 = choice_str.trim().parse().expect("");

        match choice {
            1 => {
                print!("Entr the task:  ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let _ = write_file_new(&input);
            }
            2 => {
                print!("Entr the task:  ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let _ = append_to_file(&input);
            }
            3 => {
                let _ = read_file();
            }
            _ => {
                println!("{}", Error)
            }
        };
    }
}
