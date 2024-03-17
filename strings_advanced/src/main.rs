fn main() {
    let s = String::new(); // new empty string
    let stringliteral = "hello there"; //string literal
    let string = stringliteral.to_string(); //string literal to String
    let string1 = String::from(stringliteral); //string literal to String

    let s1 = String::from("Hello");
    let s2 = String::from("There");
    let s3 = s1 + s2;
    print!("{}", s3);
    //operation above mmight seems straight forward but its not.
    // The + operator is implemented with this signature:
    // fn add(self, s: &str) -> String {
    // it expects the second argument to be a reference however it takes ownership of self which is the fist argument passed
    // it then returns the result after append self with the &str
    //however notice we are trying to pass s2 as &String instead of &str as need in the function signature and the code compiles without any issue?
    //this is because the compile can coerce &string to &str[..]
    // this operation is quite efficient as there is no copying involved;
    // it takes owenrdhip of s1 and appends s2 then returns that;
    // we can't use the s1 anymore since owenership was taken

    fn concatinate() {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s2 = String::from("toe");

        let s = s1 + "-" + s2 + "-" + s3; //seems aliitle bit verose less readble we can use format!
    }
    fn format_macro() {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s2 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}") //easier to read
    }
}
