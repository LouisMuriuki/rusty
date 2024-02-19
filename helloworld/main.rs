use std::io;
fn main() {
    loop {
        println!("Hello World");

        println!("Please guess a number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess).expect("error")
            
        let guess=match guess
            .trim()
            .parse()
            {
                Ok(num)=>num,
                Err(_)=>continue,
        };
    }
}
