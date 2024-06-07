use crate::enum_vector as enum_vectorMain;
use crate::iterator as iteratorMain;
pub mod enum_vector;
pub mod iterator;
fn main() {
    let mut vec1 = vec![1, 2, 3, 4]; // using macros
    let mut vec2: Vec<i32> = Vec::new(); //creates a new empty vector, you must explicitly type here
   // let vec3 = &vec1[0]; // this refernce might get deallocated if we modify the vector, hence rust wiill compla, get it?, as simple as it sounds:)
    vec1.push(4);
    vec2.push(4);
    // as seen we have not listed vec3 or used it, this is because vectors store items next to each other on memory and adding a new
    // element might cause new memory allocation of the vector, as such refernce to the first element or any other, might be referncing
    // a deallocated memory
    let vec2: Option<&i32> = vec1.get(2);    // get does not seem to take owbership so we can ignore the 
    // let vec2: Option<&i32> = vec1.get(2).copied().unwrap_or(0);    // get does not seem to take owbership so we can ignore the 
    // get will always return Option<&T> as it is referencing that particular item in the vector.

    print!("{:#?}", vec1);
    match vec2 {
        Some(second) => println!("we have a value{}", second),
        None => println!("we have no value"),
    }
    iteratorMain::iterator::iterate_over_print();
    iteratorMain::iterator::iterate_over_mut();

    enum_vectorMain::enum_vector::vector_enum()
}
