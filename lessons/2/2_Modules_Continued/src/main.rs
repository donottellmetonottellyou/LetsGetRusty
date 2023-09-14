use modules_continued::Credentials;

fn main() {
    let creds = Credentials::new("letsgetrusty", "password123");

    modules_continued::authenticate(creds);
}
