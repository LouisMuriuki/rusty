struct User{ //much like interfaces in js
    name:String,
    email:String,
    active:bool,
    sign_in_count: u64,
}

fn main() {
   let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
   }
  // if the instance is mutable, we can change the values of the struct
   user1.name = String::from("another name 123");

   //creating instances from other instances
   let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};
   //creating instances from other instances using struct update syntax
   let user2 = User {
    email: String::from("luihugo@gmail.com"),
    ..user1  // more like js spread operator =struct update syntax
    
};
}


// we can also create return an instance of a struct from a function
fn build_user(email:String, username:String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}