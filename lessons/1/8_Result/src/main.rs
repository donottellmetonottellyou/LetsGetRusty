fn main() {
    let username = get_username(1);

    // match username {
    //     Some(username) => println!("{username}"),
    //     None => {}
    // }
    if let Some(username) = username {
        println!("{username}");
    }
}

fn get_username(user_id: u32) -> Option<String> {
    let query = format!("GET username FROM users WHERE id={user_id}");
    let db_result = query_db(query);
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err("Query string is empty!".into())
    } else {
        Ok("Ferris".into())
    }
}
