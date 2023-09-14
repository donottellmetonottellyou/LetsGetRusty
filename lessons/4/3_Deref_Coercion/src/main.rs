use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T,
}
impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let s = MySmartPointer {
        value: Box::new("Let's Get Rusty".to_string()),
    };
    // &MySmartPointer<Box<String>> &Box<String> -> &String -> &str
    print(&s);
}

fn print(s: &str) {
    println!("{s}");
}
