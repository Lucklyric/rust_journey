use std::{fs::{File, self}, io::{ErrorKind, Read, self}};
fn main() {
    println!("Hello, world!");


    let greeting_file_result = File::open("test.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    // let greeting_file = File::open("test2.txt").unwrap();

    // let greeting_file = File::open("test3.txt").expect("Failed to open test3.txt");

    let a = 1;
    let b = 0;

    let ab = add_two_numbers(a, b);

    println!("ab: {}", ab);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => return Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("username.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

// function to add two numbers
fn add_two_numbers(a: i32, b: i32) -> i32 {
    return a + b
}
