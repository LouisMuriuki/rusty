pub mod enum_vector{
    #[derive(Debug)]
    enum SpreadSheet{
        Int(i32),
        Float(f64),
        Text(String)
        }
        pub fn vector_enum(){
               let v =vec![SpreadSheet::Int(4),SpreadSheet::Float(4.87656789),SpreadSheet::Text(String::from("Slash"))] ;
               println!("{:#?}",v)
        
        }
     
        
}