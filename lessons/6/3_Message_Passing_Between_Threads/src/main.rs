use std::{sync::mpsc, thread};

fn main() {
    let (sender, receiver) = mpsc::channel();

    let sentences = [
        "!dlroW olleH".to_string(),
        ".tsurT eW tsuR nI".to_string(),
        "!ytsuR teG s'teL".to_string(),
        "!tsuB ro tsuR".to_string(),
    ];

    for sentence in sentences {
        let sender = sender.clone();

        thread::spawn(move || {
            let sentence: String = sentence.chars().rev().collect();
            sender.send(sentence).unwrap();
        });
    }

    drop(sender);

    for sentence in receiver {
        println!("{sentence}");
    }
}
