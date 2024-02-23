fn main() {
    println!("Hello, world!");
    for_loop();
}
fn for_loop() {
    for number in (1..4).rev() { // a range of numbers not including the last one
        println!("{number}");
    }
    for number in (1..=4).rev() { // a range of numbers including the last one
        println!("{number}");
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

// fn multiloop() {
//     let mut count = 0;
//     // loop label
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn lift_off() {
//     let mut timer = 10;

//     while timer > 0 {
//         println!("{}", timer);
//         timer -= 1;
//     }
// }
