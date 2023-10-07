use attribute_like_macros_continued::log_call;

#[derive(Debug)]
struct Product {
    _name: String,
    _price: u32,
}

fn main() {
    let laptop = Product {
        _name: "MacBook Pro".into(),
        _price: 2000,
    };
    buy_product(laptop, 100);
}

#[log_call(verbose)]
fn buy_product(_product: Product, _discount: u32) {
    // ...
}
