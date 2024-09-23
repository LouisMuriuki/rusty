pub struct Post {
    state: Option<Box<dyn State>>, // trauma object to hold the state
    content: String,
}

struct Draft {}

impl State for Draft {}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})), // We set the initial state to Draft, no way to do this any other way because state is private
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> str {
        ""
    }

    pub fn request_review(&mut self){
        if let Some(s)-self.state.take(){
            self.state=s.request_review()
        }
    }
}

trait State {
    fn request_review(&mut self)-> Option<Box<dyn State>>;
}

struct PendingReview {

}
impl State for PendingReview{

}
