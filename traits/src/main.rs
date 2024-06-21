#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

struct Pair<T> {
    x: T,
    y: T,
}
pub trait Summary {
    //summarize has no default implementation
    fn summarize(&self) -> String; // method signature, no body
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// DEFAULT IMPLEMENTATION

pub trait SummaryAuthor {
    fn summarize_author(&self) -> String {
        //summarize_author has default implementation
        format!("{}", self.author)
    }
}
// no need to pass the function signature and body just specify the impl block-
//means that the NewsArticle implements the SummaryAuthor trait
impl SummaryAuthor for NewsArticle {}

fn main() {
    let article = NewsArticle {
        headline: "Techie taking over the city".to_string(),
        location: "Nairobi".to_string(),
        author: "Kimani".to_string(),
        content: "Meet Lui, Nairobi's new tech billionaire".to_string(),
    };

    println!("1 new tweet: {}", article.summarize());

    println!("Author is  {}", article.summarize_author());
}

// item is of a type that impl the Summary trait
fn this_func(item: &impl Summary) -> i32 {} //syntax sugar for below code

// can also be wriiten as a Trait bound as:
fn same_func<T: Summary>(item: &T) -> i32 {}

//multiple trait parameters
fn mutlipletrait_param(item: &impl Summary, item2: &impl Summary) -> i32 {}
// can also be wriiten as with Trait bound as:
fn same_mutlipletrait_param<T: Summary>(item: &T, item2: &T) -> i32 {}

// multiple trait bounds with + syntax
fn mutliple_trait_plus(item: &(impl Summary + Clone)) -> i32 {}

// can also be wriiten as with Trait bound on generics as
fn same_mutliple_trait_plus<T: Summary + Clone>(item: &T) -> i32 {}

fn multi_traits<T: Display + Clone, U: Display + Clone>(t: &T, u: &U) {}

// where clause multiple traits

fn multi_traits_where<T, U>(t: &T, u: &U) -> String
where
    T: Display + Clone,
    U: Display + Clone,
{
}

//returning a type that implements a trait
fn return_trait() -> impl Summary {
    NewsArticle {
        headline: "Rest in Peace Mum".to_string(),
        location: "Tanzania".to_string(),
        author: "Louis".to_string(),
        content: "Mum was involved in a road accident on 9th june 2024, in Tanzania while the car was parked aside the road, fortunately dad had just steeped out of the car and he is okay. Till we meet again MUM".to_string(),
    }
}

// using traits to implement methods

//Always implements the new method on Pair and return the pair type
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// only implements the cmp_display method on Pair if its innner type T implements PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
