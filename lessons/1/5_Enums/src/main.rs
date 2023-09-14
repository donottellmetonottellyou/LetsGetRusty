#![allow(unused)]

struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool,
}

enum ProductCategory {
    Books,
    Clothing,
    Electronics,
}

enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}
impl Command {
    fn serialize(&self) -> String {
        "{}".into()
    }
}

fn main() {
    let category = ProductCategory::Electronics;
    let product = Product {
        name: "TV".into(),
        category,
        price: 200.98,
        in_stock: true,
    };

    let cmd = Command::Undo;
    let cmd = Command::AddText("test".into());
    let cmd = Command::MoveCursor(22, 0);
    let cmd = Command::Replace {
        from: "a".into(),
        to: "b".into(),
    };

    let json_string = cmd.serialize();
}
