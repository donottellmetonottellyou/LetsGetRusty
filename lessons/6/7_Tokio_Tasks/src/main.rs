use tokio::time::sleep;

use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut handles = Vec::new();

    for i in 0..2 {
        handles.push(tokio::spawn(async move { my_function(i).await }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

// fn my_function() -> impl Future<Output = ()> {
//     ...
// }
async fn my_function(i: usize) {
    println!("[{i}]: I'm an async function!");
    let s1 = read_from_database().await;
    println!("[{i}]: First result: {s1}");
    let s2 = read_from_database().await;
    println!("[{i}]: Second result: {s2}");
}

async fn read_from_database() -> String {
    sleep(Duration::from_secs(1)).await;
    "DB Result".into()
}
