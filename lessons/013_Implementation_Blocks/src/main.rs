#![allow(unused)]

struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}
impl Product {
    fn new(name: String, price: f32) -> Self {
        Self {
            name,
            price,
            in_stock: true,
        }
    }

    fn get_default_sales_tax() -> f32 {
        0.1
    }

    fn calculate_sales_tax(&self) -> f32 {
        self.price * Self::get_default_sales_tax()
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 {
        println!("{} was bought!", self.name);
        123
    }
}

fn main() {
    let mut book = Product::new("Book".into(), 30.0);

    let sales_tax = book.calculate_sales_tax();
    println!("sales_tax: {sales_tax}");

    book.set_price(1.0);
    book.buy();

    // book.set_price(2.0);
    // borrow of moved value
}
