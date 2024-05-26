#[derive(Debug)]
pub struct NewsArticle{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

impl Summary for NewsArticle{
    fn summarize(&self)->String{   
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub trait Summary{
    //summarize has no default implementation 
    fn summarize(&self)->String;  // method signature

}

// DEFAULT IMPLEMENTATION

pub trait SummaryAuthor{
    fn summarize_author(&self)->String{  //summarize_author has default implementation 
        format!("{}", self.author)
    }
}
 // no need to pass the function signature and body just specify the impl block-
 //means that the NewsArticle implements the SummaryAuthor trait
impl SummaryAuthor for NewsArticle{
}

fn main() {

   let article=NewsArticle{
    headline:"Techie taking over the city".to_string(),
    location:"Nairobi".to_string(),
    author:"Kimani".to_string(),
    content:"Meet Lui, Nairobi's new tech billionaire".to_string(),
   };

   println!("1 new tweet: {}", article.summarize());

   println!("Author is  {}", article.summarize_author());

}

// item is of a type that impl the Summary trait
 fn func (item:&impl Summary)->i32{}

 // can also be wriiten as 
 fn same_func <T: Summary>(item:&T)->i32{}

 // multiple traits
 fn func (item:&impl Summary+Clone)->i32{}
 
 fn multi_traits <T:Display+Clone, U:Display+Clone>(t:&T, u:&U){}

  // where clause multiple traits
 fn some_func <T,U>(t:&T, u:&U)->String where T:Display+Clone, U:Display+Clone{

 }