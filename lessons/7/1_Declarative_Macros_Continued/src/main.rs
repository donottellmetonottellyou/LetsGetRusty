use declarative_macros_continued::*;

use std::collections::HashMap;

fn main() {
    hello!();

    let _scores1: HashMap<String, u32> = HashMap::new();

    let mut scores2 = HashMap::new();
    scores2.insert("Red Team".to_string(), 3);
    scores2.insert("Blue Team".to_string(), 5);
    scores2.insert("Green Team".to_string(), 2);

    let _scores3 = hash_map!(String, u32);

    let _scores4 = hash_map! {
        "Red Team".to_string() => 3,
        "Blue Team".to_string() => 5,
        "Green Team".to_string() => 2
    };
}
