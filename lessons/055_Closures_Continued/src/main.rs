#![allow(clippy::complexity)]

struct Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}
impl<T> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    fn get_default_creds(validator: T) -> Self {
        Self {
            username: "guest".into(),
            password: "password123!".into(),
            validator,
        }
    }

    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    let validator = |username: &str, password: &str| !username.is_empty() && !password.is_empty();

    //  Fn      -   Immutably borrow variables in environment.
    //  FnMut   -   Mutably borrow variables in environment: can change environment.
    //  FnOnce  -   Take ownership of variables in environment. Can only be called once.
    let weak_password = "password123!".to_owned();
    //  Fn
    let validator2 = |username: &str, password: &str| {
        validator(username, password)
            && password.len() > 8
            && password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
            && password != weak_password
    };
    println!("{weak_password}");

    let creds = Credentials {
        username: "admin".into(),
        password: "password123!".into(),
        validator: validator2,
    };

    dbg!(creds.is_valid());

    let password_validator = get_password_validator(8, true);

    let _default_creds = Credentials::get_default_creds(password_validator);
}

fn get_password_validator(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_char {
        Box::new(move |_: &str, password: &str| {
            password.len() >= min_len && password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
        })
    } else {
        Box::new(move |_: &str, password: &str| password.len() >= min_len)
    }
}
