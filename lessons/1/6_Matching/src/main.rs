#![allow(unused)]

use std::fmt::format;

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
        match self {
            Self::Undo => r#"{ "cmd": "undo" }"#.into(),
            Self::Redo => r#"{ "cmd": "redo" }"#.into(),
            Self::AddText(s) => format!(r#"{{ "cmd": "add_text", "text": "{s}" }}"#),
            Self::MoveCursor(line, column) => {
                format!(r#"{{ "cmd": "move_cursor", "line": {line}, "column": {column} }}"#)
            }
            Self::Replace { from, to } => {
                format!(r#"{{ "cmd": "replace", "from": "{from}", "to": "{to}" }}"#)
            }
        }
    }
}

fn main() {
    // let category = ProductCategory::Electronics;
    // let product = Product {
    //     name: "TV".into(),
    //     category,
    //     price: 200.98,
    //     in_stock: true,
    // };

    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText("test".into());
    let cmd3 = Command::MoveCursor(22, 0);
    let cmd4 = Command::Replace {
        from: "a".into(),
        to: "b".into(),
    };

    for cmd in [cmd1, cmd2, cmd3, cmd4]
        .into_iter()
        .map(|cmd| cmd.serialize())
    {
        println!("{cmd}");
    }

    // let json_string = cmd.serialize();

    // let age = 35;

    // match age {
    //     ..=-1 => println!("Not born yet!"),
    //     1 => println!("Happy 1st Birthday!"),
    //     13..=19 => println!("You are a teenager!"),
    //     x => println!("You are {x} years old!"),
    // }
}
