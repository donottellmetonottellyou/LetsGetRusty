#![allow(unused)]

fn main() {
    // tuples
    let rgb_color: (u8, u8, u8) = (255, 106, 0);
    let cmyk_color: (u8, u8, u8, u8) = (0, 58, 100, 0);

    // tuple structs
    struct Rgb(u8, u8, u8);
    struct Cmyk(u8, u8, u8, u8);
    let color1 = Rgb(255, 106, 0);
    let color2 = Cmyk(0, 58, 100, 0);

    // unit-like structs
    struct MyStruct;
}
