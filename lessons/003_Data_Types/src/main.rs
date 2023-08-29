#[allow(unused)]
#[allow(clippy::let_unit_value)]

fn main() {
    // ==============
    //  Atomic Types
    // ==============

    // These primative types represent scalar data, and generally cannot be
    // broken into smaller types. They usually represent a single value, such as
    // a number or a letter.

    // unit
    let unit: () = ();

    // boolean
    let b1: bool = true;

    // unsigned integers
    let i0: u8 = b'c'; // byte literals can be used
    let i1: u16 = 1;
    let i2: u32 = 1;
    let i3: u64 = 1;
    let i4: u128 = 1;

    // signed integers
    let i5: i8 = -1;
    let i6: i16 = -1;
    let i7: i32 = -1; // default integer for unspecified integer
    let i8: i64 = -1;
    let i9: i128 = -1;

    // floating point numbers
    let f0: f32 = 1.0;
    let f1: f64 = -1.0; // default floating point for unspecified f literal

    // platform specific
    let p0: usize = 1; // required for most indexing
    let p1: isize = 1;

    // unicode
    let c: char = 'c';

    // =================
    //  Composite Types
    // =================

    // These primative types represent aggregations of other types. Strings
    // contain chars (sort of), arrays contained fixed amounts of a specific
    // type, and tuples contain fixed combinations of arbitrary types.

    // unicode (continued)
    let s0: &str = "hello";
    let s1: String = "world".into();

    // arrays
    let a: [i32; 5] = [-1, 2, 5, 1_000_000, 0];
    let e0: i32 = a[3];

    // tuples
    let t: (i32, f64, char, [u8; 4]) = (-20, 2.3, 'b', [1, 2, 3, 4]);
    let e1: char = t.2;

    // =========
    //  Aliases
    // =========

    // Both primative and non-primative types can be aliased by using the `type`
    // keyword. Upper camal case is preferred for any custom types, including
    // type aliases.

    type Age = u8;

    let jorges_age: Age = 57;
}
