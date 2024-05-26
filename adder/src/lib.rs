#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
struct Guess {
    guess: i32,
}

impl Guess {
    fn new(guess: i32) {
        if guess > 100 {
            panic!("The guess is larger than 100 {}", guess)
        }
        if guess < 1 {
            panic!("The guess is smaller than 1 {}", guess)
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greetings(name: &str) -> String {
    format!("Hello {}", name)
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another(){
    //     panic!("makes this fail")
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 8,
        };
        let smaller = Rectangle {
            width: 6,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn test_greetings() {
        let name = "faith";
        assert!(
            greetings("Louis").contains(name),
            "Greetings did not contain name,value was `{}`",
            greetings(name)
        );
    }

    #[test]
    #[should_panic(expected="less than or equal to 100")]
    fn checkguess() {
        let yourguess = Guess::new(199);
        println!("{:?}", yourguess);
    }

    //test with Result<T,E>

    #[test]
    fn it_really_works()->Result<(),String>{
        if 2 -2 ==0{
            Ok(())
        }else{
            Err(String::from("two minus two is not equal to zero"))
        }


    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
