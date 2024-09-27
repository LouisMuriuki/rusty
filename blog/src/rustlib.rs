//encodes the blog post workflow into the type system as compared to the state.
pub struct Post {
    contents: String,
}

pub struct DraftPost {
    contents: String,
}

pub struct PendingReviewPost {
    contents: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { contents: String::new() }
    }

    pub fn contents(&self) -> &str {
        self.contents.as_str()
    }
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.contents.push_str(text)
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { contents: self.contents }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            contents: self.contents,
        }
    }
}
