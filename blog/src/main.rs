use crate::rustlib::Post;
mod rustlib;
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    post.approve();
}
