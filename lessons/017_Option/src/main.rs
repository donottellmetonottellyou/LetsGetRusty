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
    let db_result = String::from("Ferris");
    if user_id == 1 {
        Some(db_result)
    } else {
        None
    }
}
