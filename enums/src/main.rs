#[derive(Debug)]

enum IpAddrKind{
    V4,       // should be in camel case
    V6
}
// ability to store data as well
enum IpAddr{
    V4(String),       // should be in camel case
    V6(String),
}
#[derive(Debug)]
enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can store complex data with enums rather than structs
#[derive(Debug)]
enum IpAdd {
    V4(u8, u8, u8, u8),
    V6(String),
}

//This ussually defined in the std lib 
// it is so usefull that it is included in the prelude, we can even use the values without the option:: prefix,,check code below
// enum Option<T>{   //the type oof the enum chnages according value passed in the generic parameter
//     None,   //no value 
//     Some(T) // some value of type T  // can hold one piece of dta of nay type
// }


impl Message{
    fn call(&self)->&Message{
        self
}

}

fn main() {
    let four = IpAddrKind::V4;  //access by using a namespace
    let six = IpAddrKind::V6;
    
    let fourish = IpAddr::V4(String::from("127.0.0.1")); // the name of each enum become a function that takes a string argument and returns an instance of the associated type
    let sixish = IpAddr::V6(String::from("::1"));

    print_address(four);
    print_address(six);


    let home = IpAdd::V4(127, 0, 0, 1);

    let loopback = IpAdd::V6(String::from("::1"));

    println!("home here is {:?}", home);

    let black=Message::ChangeColor(0,0,0);

    println!("black here  is {:?}", black);

    let m = Message::Write(String::from("hello"));
    println!("m is {:#?}", m);  //returns something similar to the structure of the element in the enum with the new value given
  
    println!("this is {:#?}", m.call());  


    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    dbg!("Absent number is {:?}",&absent_number);

}

fn print_address(kind: IpAddrKind){
    println!("{:#?}", kind);

}

