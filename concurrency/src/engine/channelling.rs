use std::{ thread, sync::mpsc, time::Duration, process::exit };

//spawned_to_main is a function that demonstrates how to send a value from a spawned thread to the main thread
pub fn spawned_to_main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    println!("Received: {}", rx.recv().unwrap());
    //recv - blockes the thread currently running (Main) until a value is sent down the channel
}

pub fn multi_spawned_to_main() {
    loop {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        let tx2 = tx.clone();
        thread::spawn(move || {
            tx.send("Eat").unwrap();
            thread::sleep(Duration::from_nanos(1));
        });
        thread::spawn(move || {
            tx1.send("Sleep").unwrap();
            thread::sleep(Duration::from_nanos(1));
        });
        thread::spawn(move || {
            tx2.send("Code").unwrap();
            thread::sleep(Duration::from_nanos(1));
        });

        for received in rx {
            print!("{} ", received);
        }
    }
}
pub fn multi_spawned_to_main_with_label() {
    'about: loop {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        let tx2 = tx.clone();
        thread::spawn(move || {
            tx.send("Eat").unwrap();
            thread::sleep(Duration::from_nanos(1));
        });
        thread::spawn(move || {
            tx1.send("Sleep").unwrap();
            thread::sleep(Duration::from_nanos(1));
        });
        thread::spawn(move || {
            tx2.send("Code").unwrap();
            thread::sleep(Duration::from_nanos(1));
        });

        for received in rx {
            print!("{} ", received);
        }
        // break 'about;
    }
}
