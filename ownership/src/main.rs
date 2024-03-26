fn main() {
    //   abstraction();
    // we cannot have two mutable references in the same scope.this examples shows a scenario where it looks like we do but that isno the case
    let mut lui = "lui".to_string();
    //if we interchange the two lines we have an error
    append_muriuki(&mut lui);
    let s1 = &mut lui;
    print!("{}", s1);
}

fn append_muriuki(name: &mut String) {
    name.push_str(" Muriuki");
}

fn abstraction() {
    let mut greetings = give_ownership();
    greetings.push_str(", lui");
    let greetings2 = greetings;
    println!("{}", greetings2);

    let s3 = takesandgives_ownership(greetings2);
    println!("{}", s3);

    let s = s3; // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    let x = 5; // x comes into scope

    makes_copy(x);
    println!("{}", x); // we can still use x here, provided it's a simple scalar variable

    //return multiple values from a function using tuple
    let (length, s) = return_mutliple_values(give_ownership());
    println!("the length of {s} is {length}",);
}

fn give_ownership() -> String {
    let s1 = String::from("Hello");
    s1
}
fn takesandgives_ownership(greetings2: String) -> String {
    greetings2
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_mutliple_values(s: String) -> (usize, String) {
    let length = s.len();
    (length, s)
}
