pub mod move_to_thread;
pub mod inventory;
pub mod return_closure;

use crate::inventory::inventory_main;
use crate::move_to_thread::move_to_thread;
use crate::return_closure::return_closure;





fn main() {
    test_closures()
    // move_to_thread();
    // inventory_main()
    // test_closures()
}


fn test_closures(){
let a =vec![1,2,3,4,5];

let closure = || println!(" a is: {:?}", &a);

closure();
println!("a is: {:?}", a);
}
