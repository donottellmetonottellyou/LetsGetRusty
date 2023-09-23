use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    title: String,
    body: String,
}
impl Post {
    pub fn new(title: &str, body: &str) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
        }
    }
}
