fn main() {
    let mut s=String::from("Hello, references!");
    let length=calculate_length(&s);
    println!("The length for variable {s} is {length}");
    mutable_refences(&mut s);    
    let s1=&s.clone();
    println!("{s1}");
}

fn calculate_length(s:&String)->usize{
    let length=s.len();
    length
}

fn mutable_refences(s:&mut String){
    s.push_str(",uuuhm I meant world😅");
    println!("{s}")
}
