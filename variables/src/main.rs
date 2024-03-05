fn main() {
    let x = 5;
    let mut hello=String::from("Hello");
    let world=String::from("World");
    hello.push_str(&world);  // push_str() expects type &str

    let x = x + 1; // different from x=6

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
