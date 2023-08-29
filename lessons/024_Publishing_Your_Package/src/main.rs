use publishing_your_package::Credentials;

fn main() {
    let creds = Credentials::new("letsgetrusty", "password123");

    publishing_your_package::authenticate(creds);
}
