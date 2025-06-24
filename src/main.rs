use std::{
    fs,
    io::{Error, ErrorKind},
};

fn main() {
    let mut errors = vec![];
    match fs::read_to_string("logs.txt") {
        Ok(content) => {
            errors = extract_errors(content.as_str());
            // show them line by line
            for error in errors.as_slice() {
                println!("{}", error);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            if e.kind() == ErrorKind::NotFound {
                println!("File not found");
            }
        }
    }
    println!("Errors: {:?}", errors);
}

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");
    let mut errors = vec![];
    for line in split_text {
        if line.starts_with("ERROR") {
            errors.push(line.to_string());
        }
    }
    errors
}

fn valdidate_email(email: &str) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "Invalid email"))
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        return Err(Error::other("Cannot divide by zero".to_string()));
    }
    Ok(a / b)
}
