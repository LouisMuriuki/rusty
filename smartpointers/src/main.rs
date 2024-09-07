use crate::engine::drop::runcustomPointer;
use crate::engine::box_pointer::{ create_box_list1, create_box_list2 };
use crate::engine::rc::run_rc;
pub mod engine;
fn main() {
    run_rc()
    // create_box_list1();
    // create_box_list2()
    // runcustomPointer();

    // let b = Box::new(5);
    // println!("b ={b}");
    // main2();
}

fn main2() {}
