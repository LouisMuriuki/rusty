use std::collections::HashMap; // not included in prelude so must import
fn main() {
    // let mut scores = HashMap::new();

    // let selectedTeam = "Blue".to_string();
    // println!("Hello, world!");
    // scores.insert("Team A".to_string(), 20);
    // scores.insert("Blue".to_string(), 40);
    // scores.insert("Team C".to_string(), 50);

    // println!("{:?}", &scores);

    // // copied here will copy the variable so long as it has the copy trait else it will take ownership, we can use refernce to counter that
    // let score = scores.get(&selectedTeam).copied().unwrap_or(0); //copied will change option<&T> to Option<T> get will return Option<&T>
                                                                 //unwrap_or- will return value of key if it exits else return new value passed to key
                                                                 // -this can be alternative to match pattern

    // println!("{:?}", score);

    // iterateHashMaps(scores);
    // hashmap_overwrite();
    // hashmap_checkif_exists();
    count_word();
}

fn iterateHashMaps(map: HashMap<String, i32>) {
    for (key, value) in map {
        println!("{key}:{value},");
    }
}
// overwrite existing values
fn hashmap_overwrite() {
    let mut map = HashMap::new();
    map.insert(String::from("Kenya"), 20);
    map.insert(String::from("Tz"), 10);
    map.insert(String::from("Kenya"), 50);

    for (key, value) in map {
        println!("{key}:{value},");
    }
}
// check if it exists before add
fn hashmap_checkif_exists() {
    let mut map = HashMap::new();
    map.insert(String::from("Kenya"), 20);
    map.insert(String::from("Tz"), 10);
    map.entry("Kenya".to_string()).or_insert(50);
    // entry method check if there is naother key with the same name and return an enum (Entry) that represents a value might or might not exist.
    // entry and or_insert return a mutable refenrce to the value of the coreesponding Entry key
    println!("{:#?}", map);
}

// count words in text and  update to hashmap
fn count_word() {
    const TEXT: &str = "Hello World Wonderful World";
    let mut map = HashMap::new();
    for word in TEXT.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0); // count refers to a mutable refernce to the corresponding value of the Key
        *count += 1; // dereference the mutable reference so as to avoid an invalid pointer 
        // since count is a refernce to a value in the hashmap any change that occurs to it will affect our original value
    }
    println!("{:#?}", map); // here map will contain the new values.
}



























fn count_words() {
    let word="hello world wonderful world";
    let mut my_map=HashMap::new();

    for i in word.split_whitespace(){
        let count=my_map.entry(i).or_insert(0);
        *count+=1;
    }

    println!("{:#?}",my_map);

    
}
