struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}
impl IntoIterator for Person {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.first_name, self.last_name, self.occupation].into_iter()
    }
}

fn main() {
    let p = Person {
        first_name: "John".into(),
        last_name: "Doe".into(),
        occupation: "Software Engineer".into(),
    };

    for item in p {
        println!("{item}");
    }
}
