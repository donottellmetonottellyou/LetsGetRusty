#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

struct Database {
    max_connections: u32,
}

struct AuthService {
    db: Rc<RefCell<Database>>,
}

struct ContentService {
    db: Rc<RefCell<Database>>,
}

fn main() {
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100,
    }));
    let auth_service = AuthService { db: db.clone() };
    let content_service = ContentService { db: db.clone() };

    let mut r1 = db.borrow_mut();
    // thread 'main' panicked at 'already borrowed: BorrowMutError'
    // let r2 = db.borrow_mut();
    r1.max_connections = 200;
}
