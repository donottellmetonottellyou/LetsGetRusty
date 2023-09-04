fn main() {
    let mut s1 = String::from("Let's Get Rusty");
    let r1 = &s1;
    println!("r1: {r1}");
    // r1 not used anymore
    let r2 = &mut s1;
    r2.push('!');
}
