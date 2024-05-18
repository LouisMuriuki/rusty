fn main() {
    let longest=the_longest("abc","abcd");
    println!("The longest string is {:?}",longest);
}

// when using references as parameters rust needs to ensure that the reference returned is valid
// in this case rust does not know whether the returned reference is text1 or text2 and how it relates to the two
// hence we need to use a lifetimes
fn the_longest<'a>(text1:&'a str,text2:&'a str)->&'a str{
    if text1.len()>text2.len() {
        return text1
    }else{
        return text2
    }

}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}