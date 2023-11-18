use std::{
    env,
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    match command.as_str() {
        "new" => new_database(&args[2]),
        "open" => open_database(&args[2]),
        _ => panic!("Error: argument not found"),
    }
}

fn new_database(filename: &str) {
    if Path::new(filename).is_file() {
        panic!("Error: file already exists");
    }
    let mut file = File::create(filename).unwrap();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let password = rpassword::read_password().unwrap();

    print!("Reenter your password: ");
    io::stdout().flush().unwrap();
    let re_password = rpassword::read_password().unwrap();

    if password != re_password {
        panic!("Error: passwords don't match");
    }

    let encrypted_password = sha256::digest(password);

    write!(file, "{}", encrypted_password).unwrap();

    println!("Succesfully created new database!");
}

fn open_database(filename: &str) {
    if !Path::new(filename).is_file() {
        panic!("Error: input is not a file")
    }

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let password = rpassword::read_password().unwrap();
    let encrypted_password = sha256::digest(password);

    if encrypted_password == first_line {
        println!("correct password");
    } else {
        println!("incorrect password");
    }
}
