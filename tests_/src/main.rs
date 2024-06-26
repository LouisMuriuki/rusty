#[derive(Debug)]
struct NumberValue {
    value: i32,
}

impl NumberValue {
    fn new(some_number: i32) {
        if some_number < 5 {
            panic!("less than one")
        } else {
            println!("not less than one")
        }
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected="ssome error")] 
    fn checkif_panic() {
        NumberValue::new(10)
    }
}
