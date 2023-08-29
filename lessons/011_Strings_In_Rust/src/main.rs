#![allow(unused)]

fn main() {
    let s1: &str = "Привіт Світ! 🦀";
    let s2: String = String::from(s1);
    let s3: String = s1.to_string();
    let s4: String = s1.to_owned();
    let s5: &str = &s4[..];
    println!("{s5}");

    let mut s6: String = String::from("foo");
    s6.push_str("bar");
    println!("{s6}");
    s6.replace_range(.., "baz");
    println!("{s6}");

    let s7: String = String::from("Hello, ");
    let s8: String = String::from("world!");
    let s9: String = s7 + &s8; // s7 moved to s9
    println!("{s9}");

    let s10: String = String::from("tic");
    let s11: String = String::from("tac");
    let s12: String = String::from("toe");
    let s13: String = format!("{s10}-{s11}-{s12}");
    println!("{s13}");

    let s14: String = ["first", "second"].concat();
    let s15: &str = concat!("first", "second");

    let s16 = "🦀the🦀🦀🦀🦀";
    let s17 = &s16[0..4];
    println!("{s17}");

    for b in "नमस्ते".bytes() {
        print!("{b:03} ");
    }
    println!();

    for c in "नमस्ते".chars() {
        print!("{c} ");
    }
    println!();

    use unicode_segmentation::UnicodeSegmentation;

    for g in "नमस्ते".graphemes(true) {
        print!("{g} ");
    }
    println!();
}
