use std::ops::AddAssign;

/* State trait and structs impl State */
trait State {}

struct Draft {}
impl State for Draft {}

struct PendingReview {}
impl State for PendingReview {}

struct Published {}
impl State for Published {}

/* Blog definition and its related components */
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    pub fn new(post_content: &str) -> Post {
        return Post {
            state: Some(Box::new(Draft {})),
            content: String::from(post_content)
        };
    }

    pub fn add_content(&mut self, append_content: &str) {
        self.content.push_str(append_content);
    }
}

pub fn oop_in_rust() {
    let new_rust_post = Post::new("Hello, this is Tom updating you the news on Rust programming language");
    
}