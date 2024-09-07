
#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil
}
use super::box_pointer::List::{Nil,Cons};

pub fn create_box_list1(){
    let a = Box::new(Cons(5, Box::new(Cons(10, Box::new(Nil)))));
    println!("{:?}",a);
}

pub fn create_box_list2(){
    let b = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    println!("{:?}",b);
}