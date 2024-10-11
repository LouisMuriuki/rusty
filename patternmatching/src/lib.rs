
pub fn while_let_loop(){
    let mut stack=vec![];

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top)=stack.pop(){
        println!("{:?}",top)
    }
}

pub fn for_loop(){
    let v = vec!['a', 'b', 'c'];
//when using enumarate the order is always index, value . unlike in js 
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}