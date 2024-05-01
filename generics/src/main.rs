#[derive(Debug)]
struct Point<T,U>{
    x:T,
    y:U
}
 
// generics type defination coming the struct
impl <T,U> Point<T,U>{
//generic ty[e defination for the method to allow p2 values
    fn mixup<X1,Y2>(self,otherparam :Point<X1,Y2>)->Point<T,Y2>{ // retunr T and Y2 because we are mixing them up
        // get it ???? if not try harder you will cause I did
        Point {
            x:self.x,
            y:otherparam.y,
        }
    }
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);
    // genericstructs()
    multiple_generics()
}

// T is the type name declaraction.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest=&list[0];

    for value in list {
        if value> largest{
            largest= value
        }
    }
    largest
}


fn genericstructs(){
    let transform=Point{
        x:5,
        y:2.5
    };

    println!("Point is {:#?}",transform)

}

fn multiple_generics() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}