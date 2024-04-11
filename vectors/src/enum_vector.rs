pub mod enum_vector {
    #[derive(Debug)]

    // why enum vector?
    // vectors can only store values of the same type,
    //remember all instance of an enum are stored under a common enum type, that of the enum.
    //so basically we can have a enum with instances containing differnt types and pass that enum to the vector,
    // this makes the vector be able to hold different items with differenttypes
    enum SpreadSheet {
        Int(i32),
        Float(f64),
        Text(String),
    }
    pub fn vector_enum() {
        let v = vec![
            SpreadSheet::Int(4),
            SpreadSheet::Float(4.87656789),
            SpreadSheet::Text(String::from("Slash")),
        ];
        println!("{:#?}", v)
    }
}
