use std::ops::AddAssign;

/* State trait and structs impl State */
trait State {
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
impl State for Draft {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        return Box::new(Published {});
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
}

struct Published {}
impl State for Published {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
}

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

/* adding approval to the thing */


pub fn oop_in_rust() {
    let new_rust_post = Post::new("Hello, this is Tom updating you the news on Rust programming language");
    
}