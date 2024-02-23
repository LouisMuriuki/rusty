fn main() {
    let mut s=String::from("Hello, references!");
    let length=calculate_length(&s);
    println!("The length for variable {s} is {length}");
    mutable_refences(&mut s);    
    let s1=&s.clone();
    println!("s1 is {s1}");
    // s1.clear(); // this wont run as clear will need to changes contents of s1 -mutate but we already have an immutable reference to s1
}
    

fn calculate_length(s:&String)->usize{
    let length=s.len();
    length
}

fn mutable_refences(s:&mut String){
    s.push_str(",uuuhm I meant world😅");
    println!("{s}")
}

    //trying to return refernce of s even though its out of scope
// fn dangling_ref()->&String{ 
//     // solution is to return s instead and thus change ownership
//     let s=String::from("Hello");
//     &s
// }
