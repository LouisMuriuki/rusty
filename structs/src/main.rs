struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
#[derive(Debug)] //we must add this in-order to log the struct
struct Rectangle {
    width: u32,
    height: u32,
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
    println!("The area is {}", cal_area(4, 4));

    let dimensions = (4, 4); // tuples dont name their elements, which is height and width???
    println!("The area using tuples is {}", calc_area_tuple(dimensions));

    let rectangle = Rectangle {
        width: 4,
        height: 4,
    };

    println!(
        "The area using struct methods is {}",
        calc_area_structs(&rectangle)
    ); // we want main to retain ownership (ideally) hence the reference

    println!(
        "we can't direclty log a struct since it does not implemnt the Display trait {:#?}",
        rectangle
    ); //{:?} also works but for relatively smaller structs

    println!(
        "we can't direclty log a struct since it does not implemnt the Display trait we use this instead {:#?}",
        rectangle
    ); //{:?} also works but for relatively smaller structs

    dbg!(&rectangle); // works as println! but takes ownership, refer to notes

    // passing methods to struct instance
    println!(
        "The area using struct methods is {}",
        rectangle.calc_area_structs_methods() //note calling methods, has automatic referncing and deferencing, meaning we dont have to pass refernce to rectangle
    );

    // Methods with more parameters -by default methods have a default parameter &self
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //associated function -check implementation
    let squareRectnagle = Rectangle::square(4);
}

// we can also return an instance of a struct from a function
// fn build_user(email:String, name:String) -> User {
//     User {
//         email,
//         name,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// Area of a rectangle without structsb just ungrouped variables even though they are related

fn cal_area(width: u32, height: u32) -> u32 {
    width * height
}

// we could use structs to group them, lets see:
fn calc_area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 // tuples dont name their elements hence we have to deal with this shit!!! structs???
}

// with structs
fn calc_area_structs(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

//Defining methods in structs
impl Rectangle {
    // everything with this implementation will be associated with rectangle type
    fn calc_area_structs_methods(&self) -> u32 {
        //self here is an alias of the type the impl block is for
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width * self.height > rect.height * rect.width
    }

    // associated funct- why? it does not have a self parameter and returns a struct instance
    fn square(size: u32) -> Self {
        // Self here is an alias of the type that appears after impl keyword
        Self {
            width: size,
            height: size,
        }
    }
}
