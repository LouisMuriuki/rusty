pub fn return_closure() {
    let example_closure = |x| x;
    let louis = "Louis";
    let output = example_closure(String::from(louis));

    println!("{:?}", output)
}