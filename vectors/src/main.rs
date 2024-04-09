use crate::iterator as iteratorMain;
use crate::enum_vector as enum_vectorMain;
pub mod iterator;
pub mod enum_vector;
fn main() {
    let mut vec1 = vec![1, 2, 3, 4]; // using macros
    let mut vec2: Vec<i32> = Vec::new(); //creates a new empty vector, you must explicitly type here
    let vec3 = &vec1[0];
    vec1.push(4);
    vec2.push(4);
    // as seen we have not listed vec3 are used it, this is because vectors store items next to each other on memory and adding a new 
    // element might cause new memory allocation of the vector, as such refernce t the fisrt element or any other, might be referncing
    // a deallocated memory
    let vec2: Option<&i32> = vec1.get(2); //we can’t use the &(reference) since get will return  Option<&i32> instead of &i32

    match vec2 {
        Some(second) => println!("we have a value{}", second),
        None => println!("we have a value"),
    }
    iteratorMain::iterator::iterate_over_print();
    iteratorMain::iterator::iterate_over_mut();

    enum_vectorMain::enum_vector::vector_enum()

}
