fn main() {
    println!("Hello, world!");
    // for_loop();
    // multiloop()
}
fn for_loop() {
    for number in (1..4).rev() {
        // a range of numbers not including the last one
        println!("{number}");
    }
    for number in (1..=4).rev() {
        // a range of numbers including the last one
        println!("{number}");
    }

    for number in 1..=10 {
        println!("{}", number);
        if (number == 5) {
            break;
        }
    }
}

// fn counter() {
//     let mut x = 0;
//     let ten = 10;

//     let _result = loop {
//         if x < ten {
//             x += 1
//         } else {
//             break x * 2;
//         }
//     };
//     println!("The value of x is: {}",_result);
// }

fn multiloop() {
    let mut count = 0;
    // loop label
    'counting_up: loop {
        // loop label initialized by single quote
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // going to break the inner most loop
            }
            if count == 2 {
                break 'counting_up; //going to break the counting up loop hence it prints out End count and the code stops
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// fn lift_off() {
//     let mut timer = 10;

//     while timer > 0 {
//         println!("{}", timer);
//         timer -= 1;
//     }
// loops so far:
// -for loop
// -if..else
// -while
