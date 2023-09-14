#[allow(unused)]

fn main() {
    // creation
    let a: i16 = 5;

    // mutability
    let mut b = 5;
    b = 10;

    // shadowing
    let c = 10;
    let c = 20;

    println!("c is: {c}");

    // scope
    let d = 30;

    {
        let e = 40;
    }
    // cannot find value `e` in this scope
    // println!("inner d is: {e}");

    println!("d is: {d}");
}
