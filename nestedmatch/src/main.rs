use ::std::fs::File;
use std::io::ErrorKind;

fn main() {
    let openfile = File::open("louis.txt");

    match openfile {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("louis.txt") {
                Ok(f) => f,
                Err(e) => panic!("unexpected error {:?}", e),
            },
            other => {
                panic!("unexpected error {:?}", other);
            }
        },
    };
}
