use std::{thread, time::Duration};

fn main() {
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                for j in 0..5 {
                    thread::sleep(Duration::from_millis(1));

                    println!("Spawned thread #{i}: {j}");
                }
            })
        })
        .collect();

    for i in 0..5 {
        thread::sleep(Duration::from_millis(1));
        println!("Main thread: {i}");
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
