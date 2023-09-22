use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    let f = my_function();
    println!("Main thread waiting a second...");
    thread::sleep(Duration::from_secs(1));
    f.await;
    println!("Main thread again");
}

// fn my_function() -> impl Future<Output = ()> {
//     ...
// }
async fn my_function() {
    println!("I'm an async function!");
    let s1 = read_from_database().await;
    println!("First result: {s1}");
    let s2 = read_from_database().await;
    println!("Second result: {s2}");
}

async fn read_from_database() -> String {
    thread::sleep(Duration::from_millis(500));
    "DB Result".into()
}
