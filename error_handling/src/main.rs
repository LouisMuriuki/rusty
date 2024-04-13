use std::{fs::File, io::ErrorKind};
fn main() {
    // panic!("crash and motherfucking burn");
    could_fail()
}

// Handling Errors with result
fn could_fail() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,  //Result enum is included in prelude just like Option enum hence we dont load it as Result::Ok
        Err(error) => panic!("The program panicked because{:?}", error),
    };
}

fn could_fail_handle_multiple() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {  //ErrorKind enum not included in prelude hence we load it here
                Ok(fc) => fc,
                Err(e) => panic!("problem creatting file{:?}", e),
            },
            other_error => {
                panic!("problem creatting file{:?}", other_error)
            }
        },
    };
}
