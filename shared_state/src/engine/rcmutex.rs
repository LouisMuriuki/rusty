use std::sync::{ Arc, Mutex };
use std::thread;
pub fn multi_ownership_thread() {
    let counter = Arc::new(Mutex::new(0)); //Arc is safe for state concurrency across threads
    let mut handlers = vec![];

    for _ in 1..=10 {
        let counter = Arc::clone(&counter);  // clone the counter for ech thread
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  //remember to lock the thread, it must, its a rule
            *num += 1;     // derefence first, so as to change the actual value not pointer
        });

        handlers.push(handle);
    }
    for handle in handlers {
        handle.join().unwrap();  //mamke sure the threads dont terminate unexpectedly when one does.
    }
    println!("{:?}",counter)
}
