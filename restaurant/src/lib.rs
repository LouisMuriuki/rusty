pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    //curly brackets body of the module
    mod hosting {
        //we can place other modules inside a module
        //    can hold defination of other items e.g structs,enums, constants, traits
        fn add_to_waitlist() {}
        fn seen_at_table() {}
    }
    //servings and hosting are siblings
    mod serving {

        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
