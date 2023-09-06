use std::fs::File;

fn main() {
    let _file = File::open("example.txt").expect("Failed to open file!");
}
