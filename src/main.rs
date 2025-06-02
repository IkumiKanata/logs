use std::{fs, io::Error};

fn main() {
    let file_content = fs::read_to_string("logs.txt");
    println!("{:#?}", file_content);

    let result = divide(10.0, 0.0);
    println!("{:#?}", result);

    let result = divide(10.0, 2.0);
    println!("{:#?}", result);
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        return Err(Error::other("Cannot divide by zero".to_string()));
    }
    Ok(a / b)
}
