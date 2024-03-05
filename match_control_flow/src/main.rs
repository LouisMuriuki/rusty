#[derive(Debug)]
enum State {
    California,
    NewYork,
    Virginia,
    Ohio,
    Illinois,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        //match followed by expresion, pattern, the code to be executed
        Coin::Penny => 1, // pattern separated => operator followed by code to be executed
        Coin::Nickel => 5,
        Coin::Dime => 10, // we can use curly brackets for code spannig multiple lines
        Coin::Quarter(state) => 25,
    }
}

fn get_value_in_cents_enum(coin: Coin) -> u8 {
    match coin {
        //match followed by expresion, pattern, the code to be executed
        Coin::Penny => 1, // pattern separated => operator followed by code to be executed
        Coin::Nickel => 5,
        Coin::Dime => 10, // we can use curly brackets for code spannig multiple lines
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}", state);
            25
        }
    }
}

fn main() {
    // create an instance of the enum Coin and pass it to the function value_in_cents()
    let coin = Coin::Quarter(State::California);
    get_value_in_cents_enum(coin);
    let five = check_empty_before_add(Some(1));
    value_in_cents(Coin::Penny);
    println!("Hello, world!{:#?}",five);
    catch_all(001);
}

fn check_empty_before_add(value: Option<i8>) -> Option<i8> {
    match value {
        None => None,
        Some(value) => Some(value+1),
    }
}

fn if_let(value: Option<i8>) -> Option<i8> {
    let config_max = Some(3u8);
    if let some(max) = config_max {  // use if let instead of match when you want to check one pattern and ignore the rest - less verbose
        println!("The maximum is configured to be {}", max);
    }
}

// enum and match make a great combination to runs special functions for some values and one function for the rest.
fn catch_all(value:i8){
    match value{
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"), //represent all other values not used
        _ =>(), //represent all other values not used, here nothing will happen- represnt by unit value- the empty turple type
    }

}
