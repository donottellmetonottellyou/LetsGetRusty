use std::thread;

fn main() {
    let s = "Let's Get Rusty!".to_string();

    let handle = thread::spawn(move || {
        println!("{s}");
    });

    handle.join().unwrap();
}
