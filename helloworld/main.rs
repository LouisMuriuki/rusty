// use std::io;
fn main() {
     let string="9ii";
     let number:i32=string.trim().parse().expect("ERROR");

        println!("{}",number);
    // loop {
       
    //     println!("Hello World");

    //     println!("Please guess a number");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess).expect("error");
            
    //     let guess=match guess
    //         .trim()
    //         .parse()
    //         {
    //             Ok(num)=>num,
    //             Err(_)=>continue,
    //     };
    // }
}
