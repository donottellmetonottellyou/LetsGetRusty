pub struct Credentials {
    username: String,
    password: String,
}
impl Credentials {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}
