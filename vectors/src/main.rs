fn main() {
    let mut vec1 = vec![1, 2, 3, 4]; // using macros
    let mut vec2: Vec<i32> = Vec::new(); //creates a new empty vector, you must explicitly type here
    let vec3 = &vec1[0];
    vec1.push(4);
    vec2.push(4);
    println!("The first element is: {vec3}");

    let vec2: Option<&i32> = vec1.get(2); //we can’t use the &(reference) since get will return  Option<&i32> instead of &i32

    match vec2 {
        Some(second) => println!("we have a value{}", second),
        None => println!("we have a value"),
    }
}
