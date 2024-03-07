pub mod garden; // include code found in module garden
use crate::garden::vegetable::Asparagus;
fn main() {
    let plant=Asparagus{};
    println!("The plant is {:#?}",plant);
}
