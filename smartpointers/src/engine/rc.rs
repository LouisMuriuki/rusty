use std::rc::Rc;
use super::rc::List::{ Cons, Nil };
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn run_rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let a1 = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));
    println!(" a is {:?}", a);
    println!("a1 is {:?}", a1);

    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));

    let b1 = Rc::new(Cons(4, Rc::clone(&a)));
    println!(" b1 is {:?}", b1);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    //note there is also a weak_count which we will see later on.
}
