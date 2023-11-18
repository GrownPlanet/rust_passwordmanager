use std::{
    env,
    fs::{self, File},
    io::{self, prelude::*},
    path::Path,
};

use rand::prelude::*;

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

    print!("Create password: ");
    io::stdout().flush().unwrap();
    let password = rpassword::read_password().unwrap();

    print!("Reenter your password: ");
    io::stdout().flush().unwrap();
    let re_password = rpassword::read_password().unwrap();

    if password != re_password {
        panic!("Error: passwords don't match");
    }

    let mut rng = rand::thread_rng();
    let salt: i32 = rng.gen();

    let encrypted_password = sha256::digest(format!["{}{}", salt, password]);

    writeln!(file, "{}", encrypted_password).unwrap();
    writeln!(file, "{}", salt).unwrap();

    println!("Succesfully created new database!");
}

fn open_database(filename: &str) {
    if !Path::new(filename).is_file() {
        panic!("Error: input is not a file")
    }

    let file = fs::read_to_string(filename).unwrap();
    let mut lines = file.lines();
    let pass = lines.next().unwrap();
    let salt = lines.next().unwrap();

    print!("Enter password: ");
    io::stdout().flush().unwrap();

    let input_password = rpassword::read_password().unwrap();
    let input_encrypted_password = sha256::digest(format!["{}{}", salt, input_password]);

    if input_encrypted_password == pass {
        println!("correct password");
    } else {
        println!("incorrect password");
    }
}
