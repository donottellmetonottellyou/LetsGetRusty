#![allow(unused)]

pub fn draw_line(x: i32, y: i32) {
    // draw line without color
}

#[cfg(feature = "color")]
pub mod color {
    pub use rgb::RGB;

    pub fn draw_line(x: i32, y: i32, color: &RGB<u8>) {
        // draw line with color
        println!("{color}");
    }
}

#[cfg(feature = "shapes")]
pub mod shapes {
    use rgb::RGB;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Rectangle {
        pub color: RGB<u8>,
        pub width: u32,
        pub height: u32,
    }
}
