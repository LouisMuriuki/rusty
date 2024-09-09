use crate::engine::{
    threadding::run_thread,
    channelling::{ spawned_to_main, multi_spawned_to_main },
};

pub mod engine;
fn main() {
    // run_thread()
    // spawned_to_main();
    multi_spawned_to_main()
}
