use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("red team".into(), 2);
    scores.insert("blue team".into(), 8);
    scores.insert("green team".into(), 6);

    for (team, score) in &scores {
        println!("{team} got {score} points");
    }

    for (team, score) in &mut scores {
        *score += 1;
        println!("{team} got {score} points");
    }

    for (team, score) in scores {
        println!("{team} got {score} points; dropping!");
    }
}
