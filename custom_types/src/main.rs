use std::io;
struct Guess{
    value:i32,
}

//use a custom type to limit your variable to some certain condition, have a check in the custom function for this conditions 
//when creating an instance of the type from the value recieved.

impl Guess{
    fn new (value:i32)->Guess{
        if value<1 ||value>100{
            panic!("The value is not in range 1..100 {}",value);
        }
        Guess {value}
    }
// i feel like this is the part i might forget, will go back and refer to structs.
    fn value_getter(&self)->i32{
        self.value
    }
}

fn main() {
    loop{
        println!("Enter a number between 0-100");
        let mut guess=String::new();
        io::stdin().read_line(&mut guess).expect("Some io error occured");
        let value_type:i32=guess.trim().parse().expect("error");
        let value:Guess =Guess::new(value_type);
        let actual_value:i32=value.value_getter();
        println!("You entered{}",actual_value);
    }
}
    