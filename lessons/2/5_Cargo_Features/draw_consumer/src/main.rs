#![allow(unused)]

use cargo_features::color;

fn main() {
    cargo_features::draw_line(1, 1);

    let color = color::RGB {
        r: 255,
        g: 78,
        b: 3,
    };

    color::draw_line(18, 18, &color);

    let square = cargo_features::shapes::Rectangle {
        color,
        width: 32,
        height: 12,
    };
}
