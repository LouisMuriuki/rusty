use std::sync::Mutex;

use crate::engine::rcmutex::multi_ownership_thread;
pub mod engine;
fn main() {
    multi_ownership_thread()
    // run_main();


}

fn run_main() {
    let m = Mutex::new(7);
    println!("m = {:?}", m);

    {
        let mut num = m.lock().unwrap();
        *num = 10;
    }
    println!("m = {:?}", m);
}
