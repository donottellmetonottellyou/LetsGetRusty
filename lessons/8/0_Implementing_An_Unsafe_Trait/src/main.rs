#[allow(clippy::missing_safety_doc)]
unsafe trait MyTrait {
    fn some_function(&self);
}

unsafe impl MyTrait for String {
    fn some_function(&self) {
        // ...
    }
}

fn main() {}
