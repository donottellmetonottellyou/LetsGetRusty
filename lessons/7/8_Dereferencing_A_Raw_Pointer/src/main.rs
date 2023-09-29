fn main() {
    let mut s = "Let's Get Rusty".to_string();

    let raw1 = &s as *const String;
    let raw2 = &mut s as *mut String;

    let address: usize = 0x012345678;
    let raw3 = address as *const String;

    unsafe {
        (*raw2).push_str("!!!");
        println!("raw1 is: {}", *raw1);

        // Undefined
        println!("raw3 is: {}", *raw3);
    }
}
