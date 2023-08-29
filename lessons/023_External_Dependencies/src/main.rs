use external_dependencies::Credentials;

fn main() {
    let creds = Credentials::new("letsgetrusty", "password123");

    external_dependencies::authenticate(creds);
}
