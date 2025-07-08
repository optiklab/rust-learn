// The implementation using the state pattern is easy to extend to add more functionality. 

// Downsides of the state pattern are:
// 1. Because the states implement the transitions between states, 
// some of the states are coupled to each other. If we add another state between
//  PendingReview and Published, such as Scheduled, we would have to change the 
// code in PendingReview to transition to Scheduled instead. It would be less
//  work if PendingReview didn’t need to change with the addition of a new state, 
// but that would mean switching to another design pattern.
//
// 2. We’ve duplicated some logic. To eliminate some of the duplication, 
// we might try to make default implementations for the request_review and 
// approve methods on the State trait that return self; however, 
// this wouldn’t work: when using State as a trait object, the trait doesn’t 
// know what the concrete self will be exactly, so the return type isn’t known 
// at compile time. (This is one of the dyn compatibility rules mentioned 
// earlier.)


fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

// Why Not An Enum?
// One disadvantage of using an enum is that every place that checks the value 
// of the enum will need a match expression or similar to handle every possible 
// variant. This could get more repetitive than this trait object solution.
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        //""
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}