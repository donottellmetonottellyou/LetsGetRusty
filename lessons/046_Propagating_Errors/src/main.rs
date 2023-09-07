use std::{fs, io, io::Read};
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let _contents = read_file("example.txt");
    let user = User {
        firstname: "Jade".into(),
        lastname: "Masker".into(),
    };
    let _initials = user.get_initials();
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    fs::File::open(filename)?.read_to_string(&mut contents)?;

    Ok(contents)
}

struct User {
    firstname: String,
    lastname: String,
}
impl User {
    fn get_initials(&self) -> Option<String> {
        let first = self.firstname.graphemes(true).next()?;
        let last = self.lastname.graphemes(true).next()?;
        Some(format!("{first}.{last}."))
    }
}
