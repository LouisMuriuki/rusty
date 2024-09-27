

// //the not so rust way of doing things - OOP
// pub struct Post {
//     state: Option<Box<dyn State>>, // trait object to hold the state
//     content: String,
//     approvalTimes:i32,
// }

// impl Post {
//     pub fn new() -> Self {
//         Post {
//             state: Some(Box::new(Draft {})), // We set the initial state to Draft, no way to do this any other way because state is private
//             content: String::new(),
//             approvalTimes:0
//         }
//     }

//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }

//     pub fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)
//     }

//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             //here we call the approve mrthod on the current state item
//             self.state = Some(s.request_review());
//         }
//     }

//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             //here we call the approve mrthod on the current state item
//             self.state = Some(s.approve());

//         }
//     }

//     pub fn reject(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.reject());
//         }
//     }
// }

// trait State {
//     // &mut self same as self: &mut self
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;

//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State>;
// }

// struct Draft {}

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }

// struct PendingReview {}
// impl State for PendingReview {
//     // method only valid to a box holding the type.
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }

//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Draft {})
//     }
// }

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     // default impl returns empty string for every state, we override that here by returning the posts contents.
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Draft {})
//     }
// }
