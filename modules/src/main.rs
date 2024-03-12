pub mod garden; // include code found in module garden
use crate::garden::vegetable::Asparagus;
fn main() {
    let plant=Asparagus{};
    println!("The plant is {:#?}",plant); // prints Asparagus instead of {} because of the way the debug trait works and since they are no values inside the struct
}
