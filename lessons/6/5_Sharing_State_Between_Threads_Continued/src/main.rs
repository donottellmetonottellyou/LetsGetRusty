use std::{
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug)]
struct Database {
    connections: Vec<u32>,
}
impl Database {
    fn new() -> Self {
        Self {
            connections: Vec::new(),
        }
    }

    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}

fn main() {
    let db = Arc::new(Mutex::new(Database::new()));

    let mut handles = Vec::new();

    for i in 0..10 {
        let db = db.clone();
        handles.push(thread::spawn(move || {
            let mut db_lock = db.lock().unwrap();
            db_lock.connect(i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", db.lock().unwrap().connections);
}
