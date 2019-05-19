//mod blog;
mod blog2;
//use blog::Post;

use blog2::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate salad for lunch today", post.content());

}
