use std::net::IpAddr;
use std::{
    fs::{self, read_to_string, File},
    io::{self, ErrorKind, Read},
};
fn main() {
    // panic!("crash and motherfucking burn");
    // could_fail()
    let v=vec![1,2,3,4,5];
    let v10=v[99];
}

// Handling Errors with result
fn could_fail() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file, //Result enum is included in prelude just like Option enum hence we dont load it as Result::Ok
        Err(error) => panic!("The program panicked because{:?}", error),
    };
}

fn could_fail_handle_multiple() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                //ErrorKind enum not included in prelude hence we load it here
                Ok(fc) => fc,
                Err(e) => panic!("problem creatting file{:?}", e),
            },
            other_error => {
                panic!("problem creatting file{:?}", other_error)
            }
        },
    };
}

fn could_fail_handle_multiple_simplified() {
    let greeting_file_result = File::open("hello.txt").unwrap();
    print!("{:#?}", greeting_file_result)
}
fn could_fail_handle_multiple_simplified_error() {
    let greeting_file_result =
        File::open("hello.txt").expect("Hello.txt is npot included in this project!");
    print!("{:#?}", greeting_file_result)
}

//Propagating Errors- returns an Error to the code that called our code
//Result<T,E> - T & E are generics parameters
//if code succeds, calling code will recieve an Ok value that holds a String,else it will recieve io::Error that contains more information about what the problem was
//we chose io:Error as the return type of this function because that happens to be the type of the error value returned from both of the operations were calling in this function body that might fail
//the File::open function and the read_to_string method.
fn read_username_from_file() -> Result<String, io::Error> {
    let read_username = File::open("read.txt");

    let mut username_file = match read_username {
        Ok(file) => file, // once reading file is successful file handle becomes the value in in mut variable username_file
        Err(e) => return Err(e), // once reading file is unsuccessful we early return out of the function and pass error value from file::open to the calling function
    };

    let mut username = String::new(); //create new string if  username_file is available ofcos

    //read contents of file to username variable.
    match username_file.read_to_string(&mut username) {
        //read_to_string returns a result because it might fail.hence the need for a match.
        Ok(_) => Ok(username),
        Err(e) => Err(e), // no need to pass return since this is the last expression in our function.
    }
}

fn read_username_from_file_simplified() -> Result<String, io::Error> {
    let mut username_file = File::open("somepath_here")?; //if Ok returned value inside ok will be returned
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_even_simpler() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("somepath")?.read_to_string(&mut username)?;
    Ok(username)
}

//even easier way to do it.

fn read_from_file_godmode() -> Result<String, io::Error> {
    fs::read_to_string("somepathhere")
    // reading a file into a string is a fairly common operation, hence the std lib provides convinient fs::read_to_string() that opens
    // the file, creates a new string, reads content of a file, puts the content into that string and returns it.
    // using fs:read_to_string(path) did not give us the ability to explain error handling hence why we did't use it
}

// etrun type is Option<T> because there are chances the character is there or not
fn find_lastcharacter_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    //.lines -returns an iterator over the lines in the string.
    //.next - Because this function wants to examine the first line, it calls next on the iterator to get the first value from the iterator
    // if the text is an empty string then this call will return None() in which case we use ? to stop and return None from the fn
    // if text is not an empty string next will return a some value containg a string slice of the first line in the text.
    // we can now call chars to get an iterator of the String slice.
    // we then call last to return the last item in the iterator.
}

fn dummy_compiler() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Please enter a valid Ip Address");
}
