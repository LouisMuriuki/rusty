use std::fs::File;
fn main() {
    // panic!("crash and motherfucking burn");
    could_fail()
}

// Handling Errors with result
fn could_fail() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file= match greeting_file_result {
        Ok(file)=>file,
        Err(error)=>panic!("The program panicked because{:?}",error),
    };
}
