#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

    // Infinite loop for running
    loop {
        //Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut args_iter = input.trim().split(' ').into_iter();
        let command = args_iter.next();
        match command {
            // in future, get args
            Some(c) => match c {
                "" => continue,
                // hacky, but we'll implement this properly later
                "exit" => match args_iter.next() {
                    Some("0") => std::process::exit(0),
                    _ => {}
                },
                "type" => match args_iter.next() {
                    Some(s) => match s {
                        builtin if
                            builtin == "exit" || builtin == "type" || builtin == "echo"
                            => println!("{} is a shell builtin", builtin),
                        other => println!("{}: not found", other),
                    },
                    _ => {},
                },
                "echo" => {
                    for v in args_iter {
                        print!("{} ", v);
                    }
                    println!("");
                },
                _ => {
                    println!("{}: command not found", input.trim());
                },
            }
            None => {
                continue;
            }
        }
    }
}
