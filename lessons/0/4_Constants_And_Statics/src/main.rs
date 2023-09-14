#![allow(unused)]

const MAX_PLAYERS: u8 = 10;
static CASINO_NAME: &str = "Super Mario Casino";

fn main() {
    let a = MAX_PLAYERS;
    // changes to:
    // let a = 10;
    // so there isn't a specific address where 10 is

    let b = CASINO_NAME;
    // assigns b a pointer to CASINO_NAME

    // Use constants unless:
    //  -   The data is large.
    //  -   You need a single address.
    //  -   You need interior mutability.
}
