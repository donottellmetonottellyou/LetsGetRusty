#![allow(unused)]

struct BrowserCommand<T> {
    name: String,
    payload: T,
}
impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        Self { name, payload }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn main() {
    let cmd1 = BrowserCommand::new(
        "navigate".to_string(),
        "https://www.letsgetrusty.com".to_string(),
    );
    cmd1.print_payload();
    let p1 = cmd1.get_payload();
    let s1 = serialize_payload(&p1);

    let cmd2 = BrowserCommand::new("zoom".to_string(), 200);
    // cmd2.print_payload();
    let p2 = cmd1.get_payload();
    let s2 = serialize_payload(&p2);
}

fn serialize_payload<T>(payload: &T) -> String {
    // convert payload to JSON string...
    "placeholder".to_string()
}
