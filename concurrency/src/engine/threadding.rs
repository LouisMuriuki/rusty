use std::{ thread, time::Duration, vec };
// Calling join on the handle blocks the thread currently running until the thread
//  represented by the handle terminates.
pub fn run_threads() {
    let multi = thread::spawn(|| {
        for i in 1..=10 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //join blocks  the thread currently running untill the thread multi terminates
    //then it can proceed to the next thread
    // i.e if we put the join.unwrap statement at the end of the main fn, it will print
    // interchangbly, and the main thread will have to wait after finising before exting
    // however if we leave it as is,, it will print all the value in mutli first then the main thread

    for i in 1..19 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    multi.join().unwrap();
}

pub fn run_thread(){

    let a = vec![1,2,3,4,5];
    thread::spawn(move || println!("a is: {:?}", a)).join().unwrap();
}
