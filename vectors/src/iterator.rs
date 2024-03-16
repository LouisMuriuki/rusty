pub mod iterator {
    // iterate and just print
    pub fn iterate_over_print() {
        let some_vector = vec![1, 2, 3, 4, 5, 6, 7];
        // if we dont pass a refence here the ownership is transfered to the  for loop and cannot be accessed, essentially it;s up to you to fugure out how you wan tyour pprogram to run
        for i in some_vector {
            println!("{i}")
        }
    }

    //iterate and mutate the vector
    pub fn iterate_over_mut() {
        let mut v = vec![100, 60, 50, 40, 90];
        //same as above here
        for i in &mut v {
            *i = *i+50 //* dereference operator, allows us to get the value from the refernce value i
          //  *i+=50  valid as well
        }
        print!("{:#?}", v);
    }
}
