fn main() {
    let v = ["one", "two", "three"];

    #[allow(unconditional_panic)]
    let fourth = v[3];
    println!("{}", fourth);
}
