use std::process::Command;
#[allow(unused_imports)]
use std::{
    env,
    io::{self, Write},
};

mod parser;

static BUILTINS: phf::Map::<&str, fn(args: parser::SplitArgsIter, path: &std::collections::HashMap<String, String>) -> ()> = phf::phf_map! {
    "cd" => |mut args, _| {
        let arg = args.next();
        match arg {
            // Change this to allow paths other than ~
            Some(home) if home.as_str() == "~" => {
                let home = match env::consts::OS {
                    "windows" => {
                        env::var_os("UserProfile")
                    },
                    _ => {
                        env::var_os("HOME")
                    }
                };
                match home {
                    Some(v) => {
                        let path = v.to_str();
                        if path.is_some() {
                            match env::set_current_dir(path.unwrap()) {
                                Ok(_) => {},
                                Err(e) => match e.raw_os_error() {
                                    // windows: no file
                                    Some(2) => println!("cd: {}: No such file or directory", path.unwrap()),
                                    _ => println!("error: {}", e),
                                },
                            }
                        }
                    },
                    None => println!("no home variable!!!!"),
                }
            },
            Some(path) => match env::set_current_dir(path.as_str()) {
                Ok(_) => {},
                Err(e) => match e.raw_os_error() {
                    // windows: no file
                    Some(2) => println!("cd: {}: No such file or directory", path),
                    _ => println!("error: {}", e),
                },
            }
            None => {},
        }
    },
    "echo" => |args, _| {
        for v in args {
            print!("{} ", v);
        }
        println!("");
    },
    "exit" => |mut args, _| {
        match args.next() {
            Some(zero) if zero.as_str() == "0" => std::process::exit(0),
            _ => {}
        };
    },
    "pwd" => |_args, _| {
        match env::current_dir() {
            Ok(path) => match path.as_os_str().to_str() {
                Some(s) => println!("{}", s),
                None => println!("ewww ascii"),
            },
            Err(err) => println!("error: {}", err),
        }
    },
    "type" => |mut args, path| {
        match args.next() {
            Some(s) => match s.as_str() {
                builtin if BUILTINS.contains_key(builtin) => {
                    println!("{} is a shell builtin", builtin)
                }
                other => match path.get(other) {
                    Some(path) => println!("{} is {}", other, path),
                    None => println!("{}: not found", other),
                },
            },
            _ => {}
        }
    },
};

fn main() {
    // This doesn't really feel good, but we'll deal with this later.
    let mut path_map = std::collections::HashMap::new();
    //setup PATH
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let dir_result = std::fs::read_dir(path);
                match dir_result {
                    Ok(dir_iter) => {
                        for file_result in dir_iter {
                            match file_result {
                                Ok(entry) => {
                                    let path = entry.path();
                                    let stem = path.file_stem().unwrap().to_str();
                                    if stem.is_some() {
                                        let name = stem.unwrap();
                                        if !path_map.contains_key(name) {
                                            path_map.insert(
                                                name.to_owned(),
                                                path.as_os_str().to_str().unwrap().to_owned(),
                                            );
                                        }
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    // Windows error: file not found
                    Err(e) if e.raw_os_error() == Some(3) => {}
                    Err(e) => println!("error: {}", e),
                }
            }
        }
        None => println!("no path?????"),
    }

    // Infinite loop for running
    loop {
        //Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut args_iter = parser::SplitArgsIter::new(input.trim());
        let command = args_iter.next();
        match command {
            Some(c) => match c.as_str() {
                "" => continue,
                builtin if BUILTINS.contains_key(builtin) => {
                    BUILTINS.get(builtin).unwrap()(args_iter, &path_map);
                }
                // TODO: we don't need to get the path, that's expensive
                other => match path_map.contains_key(other) {
                    true => {
                        let result = Command::new(other)
                            .stdout(io::stdout())
                            .stderr(io::stderr())
                            .args::<Vec<String>, String>(args_iter.collect()).output();
                        match result {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{}", e);
                            },
                        }
                    }
                    false => println!("{}: command not found", input.trim()),
                },
            },
            None => {
                continue;
            }
        }
    }
}
