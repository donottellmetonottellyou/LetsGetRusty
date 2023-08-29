#![allow(unused)]

struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

fn main() {
    let mut book = Product {
        name: String::from("Book"),
        price: 28.85,
        in_stock: true,
    };

    let sales_tax = calculate_sales_tax(&book);
    println!("sales_tax: {sales_tax}")
}

fn calculate_sales_tax(product: &Product) -> f32 {
    product.price * 0.1
}
