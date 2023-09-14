use std::{fs, io, num};

fn main() {
    let i = parse_file("example.txt");

    match i {
        Ok(i) => println!("{i}"),
        Err(e) => match e {
            ParseFileError::File(e) => println!("File io error: {e}"),
            ParseFileError::Parse(e) => println!("Parsing error: {e}"),
        },
    }
}

enum ParseFileError {
    File(io::Error),
    Parse(num::ParseIntError),
}

fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename).map_err(ParseFileError::File)?;
    let i = s.parse().map_err(ParseFileError::Parse)?;
    Ok(i)
}
