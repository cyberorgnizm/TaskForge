use std::{
    fs::File,
    io::{self, Read},
};

// Error handling
pub fn error_handling() {
    let ans = division(12, 6).expect("Failed to divide");
    println!("Division = {ans}");

    open_file();
    read_username_from_file().expect("Error reading username");
    read_username_from_file_v2().expect("Error reading username from file");
}

pub fn open_file() {
    let file = File::open("example.txt");

    if let Ok(mut val) = file {
        let mut string = String::new();
        let _ = val.read_to_string(&mut string);
        println!("Content here: {string}");
    }
}

pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("names.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

pub fn division(a: u32, b: u32) -> Result<u32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    return Ok(a / b);
}
