#![allow(unused)]

use custom_derive_procedural_macros::*;

trait Log {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Debug, Log)]
struct Database {
    url: String,
    connections: u32,
}
impl Database {
    fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            connections: 0,
        }
    }

    fn connect(&mut self) {
        self.info(&format!("New connection to {}", self.url));
        self.connections += 1;

        if self.connections >= 100 {
            /// a lot of connections!
            self.warn(&format!(
                "{} is a lot of connections open!",
                self.connections
            ));
        }
    }
}

fn main() {
    let mut db = Database::new("localhost:3001");
    for _ in 0..100 {
        db.connect();
    }
}
