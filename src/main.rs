#[allow(unused_imports)]
use std::{io::{self, Write}, env};

fn main() {
    // This doesn't really feel good, but we'll deal with this later.
    let mut path_map = std::collections::HashMap::new();
    //setup PATH
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let d = std::fs::read_dir(path);
                match d {
                    Ok(dir_iter) => for file_res in dir_iter {
                    match file_res {
                        Ok(entry) => {
                            let path = entry.path();
                            let name = path.file_stem().unwrap().to_str();
                            if name.is_some() {
                                path_map.insert(
                                    name
                                        .unwrap()
                                        .to_owned(),
                                    "path path".to_owned(),
                                );
                            }
                        },
                        Err(_) => {}
                    }
                }
                    // Windows error: file not found
                    Err(e) if e.raw_os_error() == Some(3) => {},
                    Err(e) => println!("error: {}", e),
                }
            }
        }
        None => println!("{key} is not defined in the environment.")
    }

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
