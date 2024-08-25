pub mod move_to_thread;
pub mod inventory;
pub mod return_closure;

use crate::inventory::inventory_main;
use crate::move_to_thread::move_to_thread;
use crate::return_closure::return_closure;





fn main() {
    move_to_thread();
    // inventory_main()
    // test_closures()
}
