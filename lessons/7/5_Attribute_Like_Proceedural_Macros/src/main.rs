#![allow(unused)]

use attribute_like_proceedural_macros::log_call;

#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
}

fn main() {
    let laptop = Product {
        name: "MacBook Pro".into(),
        price: 2000,
    };
    buy_product(laptop, 100);
}

#[log_call(verbose)]
fn buy_product(_product: Product, _discount: u32) {
    // ...
}
