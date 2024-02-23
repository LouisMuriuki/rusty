fn main() {
    let s1=String::from("Louis Muriuki");
    let index=first_word_legacy(&s1); // immutable reference used here
    println!("index is {index}");
    let text=first_word_legacy_slices(&s1);
    first_word();
    println!("first word is {text}");
    let mystring_literal="Louis Muriuki";
    let first_wad=first_word_slices(&s1[..5]); //slice from s1 of type String
    let first_wad_literal=first_word_slices(&mystring_literal[..5]); // slice from mystring literal of type str
    println!("first wad is {first_wad}");
    println!("first wad Literal is {first_wad_literal}");
    array_slice();
}

//  Slice -
// a type of reference that allows us to reference a subset of elements from a collection
// instead of referencing the whole collection
fn first_word(){
    let s=String::from("Hello world!");
    let hello=&s[0..5]; //slices first word included, last not included
    let world=&s[6..=11];
    println!("{hello}{world}");
}

// here we return index of the space character and we can use that to extract the word
fn first_word_legacy(s:&String)->usize{
    let array=s.as_bytes();

    for (i,&item) in array.iter().enumerate(){
        if item==b' '{
            return i;
        }
    }

s.len()
}

// here we return the actual first word using slices
// i.e if we get the last index we can extract a slice of the string
fn first_word_legacy_slices(s:&String)->&str{
    let array=s.as_bytes();

    for (i,&item) in array.iter().enumerate(){
        if item==b' '{
            return &s[..i]
        }
    }

&s[..]
}

// see how accepting a &str instead of a &String reduces the amount of code we need to write
// we also can pass data from String or string literals to a function that accepts &str 
//we must however remember to pass a string literal instead of a string
fn first_word_slices(s:&str)->&str{
    s
}

fn array_slice(){
    let a=[1,2,3,4,5];
    let slice=&a[1..3];
    println!("{slice:?}");
}




