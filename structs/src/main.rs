struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // tuple_struct
fn main() {
    let black = Color(0, 0, 0);
    
    let mut user1 = User {
        active: true,
        name: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // if the instance is mutable, we can change the values of the struct
    user1.name = String::from("another name 123");

    //creating instances from other instances
    let user2 = User {
        active: true,
        name: String::from("lui"),
        email: String::from("another@example.com"),
        sign_in_count: 10,
    };
    println!("{}", user1.name);
    //creating instances from other instances using struct update syntax
    // we use the assignment operator to show that that data is being moved,
    //however the user1 will become invalid if we spread data on it stored on the heap i.e (Strings,Compound types) to another struct
    //however if we spread only scalar datatype, user1 is still valid, check the code below
    // if we just use the struct as a type user1 is still valid, check the code above
    let user3 = User {
        email: String::from("luihugo@gmail.com"),
        name: String::from("luihugo"), //removing this means we are spreading a String type hence user1 become invalid
        ..user1                        // more like js spread operator, must come last though
    };

    println!("{}", user1.name); //user already moved, hence it is inavalid
}

// we can also create return an instance of a struct from a function
// fn build_user(email:String, name:String) -> User {
//     User {
//         email,
//         name,
//         active: true,
//         sign_in_count: 1,
//     }
// }
