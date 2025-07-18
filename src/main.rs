use std::{
    fs,
    io::{Error, ErrorKind},
};

fn main() {
    // match fs::read_to_string("logs.txt") {
    //     Ok(content) => {
    //         match fs::write("errors.txt", extract_errors(content.as_str()).join("\n")) {
    //             Ok(..) => println!("Errors written to file"),
    //             Err(e) => println!("Error writing to file: {}", e),
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error: {}", e);
    //         if e.kind() == ErrorKind::NotFound {
    //             println!("File not found");
    //         }
    //     }
    // }
    let text = fs::read_to_string("logs.txt").expect("Failed to read file");
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n")).expect("Failed to write to file");
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
