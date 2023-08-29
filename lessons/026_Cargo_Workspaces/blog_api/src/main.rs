use blog_shared::Post;

fn main() {
    let post = Post::new("Post on the server", "Let's get rusty!");

    println!("{post:?}");
}
