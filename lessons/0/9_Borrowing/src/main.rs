fn main() {
    let mut s1 = String::from("Rust"); // heap allocated string

    print_string(&s1);

    // Non-lexical lifetimes:
    // r2's lifetime starts here
    let r2 = &mut s1;
    add_to_string(r2);
    // r2's lifetime ends here

    // borrowing s1 is now valid because r2 is not being used
    print_string(&s1);

    let _s2 = generate_string();
}

fn generate_string() -> String {
    String::from("Ferris")
}

fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome!")
}

fn print_string(p1: &String) {
    println!("{p1}");
}
