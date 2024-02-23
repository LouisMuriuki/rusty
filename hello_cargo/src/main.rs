fn five()->i8{
    5
}

fn return_five(){
    let x = five();
    println!("The value of x is: {}", x);
}


fn main() {
 return_five();
