#![allow(clippy::never_loop)]

fn main() {
    // if/else
    let a = 5;

    if a > 5 {
        println!("bigger than 5");
    } else if a > 3 {
        println!("bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }

    let _b = if a > 5 { 1 } else { -1 };

    // loops
    let _x = 'outer: loop {
        println!("loop forever");
        loop {
            break 'outer 8;
        }
    };

    // whiles
    let mut a = 0;

    while a < 5 {
        println!("a is {a}");
        a += 1;
    }

    // fors
    let a = [1, 2, 3, 4, 5, 10];

    for e in a {
        println!("{e}");
    }
}
