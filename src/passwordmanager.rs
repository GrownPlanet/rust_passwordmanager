use std::{
    env,
    fs::{self, File},
    io::{self, prelude::*},
    path::Path,
};

use rand::prelude::*;

use dialoguer::{Confirm, Password};

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    match command.as_str() {
        "new" => {
            if args.len() > 2 {
                new_database(&args[2]);
            } else {
                println!("Error: no database suplied");
            }
        }
        "open" => {
            if args.len() > 2 {
                open_database(&args[2])
            } else {
                println!("Error: no database suplied");
            }
        }
        _ => panic!("Error: argument not found"),
    }
}

fn new_database(filename: &str) {
    if Path::new(filename).is_file() {
        panic!("Error: file already exists");
    }
    let mut file = File::create(filename).unwrap();

    let password = Password::new()
        .with_prompt("Create password")
        .with_confirmation("Confirm your password", "Passwords don't match")
        .interact()
        .unwrap();

    let mut rng = rand::thread_rng();
    let salt: i32 = rng.gen();

    let encrypted_password = sha256::digest(format!["{}{}", salt, password]);

    writeln!(file, "{}", encrypted_password).unwrap();
    writeln!(file, "{}", salt).unwrap();

    println!("Succesfully created new database!");
}

fn open_database(filename: &str) {
    if !Path::new(filename).is_file() {
        println!("Error: database does not exist");
        if !ask_to_create(filename) {
            panic!("No database");
        }
    }

    let file = fs::read_to_string(filename).unwrap();
    let mut lines = file.lines();

    if lines.clone().count() < 2 {
        panic!("File is too short, no salt/ password stored");
    }

    let encrypted_password = lines.next().unwrap();
    let salt = lines.next().unwrap();

    print!("Enter password: ");
    io::stdout().flush().unwrap();

    let input_password = Password::new()
        .with_prompt("Enter password")
        .interact()
        .unwrap();

    let input_encrypted_password = sha256::digest(format!["{}{}", salt, input_password]);

    if input_encrypted_password == encrypted_password {
        println!("correct password");
    } else {
        println!("incorrect password");
    }
}

fn ask_to_create(filename: &str) -> bool {
    if Confirm::new()
        .with_prompt("Create new database?")
        .interact()
        .unwrap()
    {
        new_database(filename);
        return true;
    }

    false
}
