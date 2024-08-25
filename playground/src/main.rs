fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let extracted = extract(first_sentence);
}
#[derive(Debug, Clone)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl <'a>ImportantExcerpt<'a> {
    fn extract(part: &'a str){
        println!("{:?}", &part[0..5]);

    }
}

fn order(sentence: &str) -> String {
    let word_vec: Vec<&str> = sentence.split(" ").collect();
    let mut tuple_vec: Vec<(char, &str)> = Vec::new();
    let mut processed_vec = Vec::new();
    let mut final_string = String::new();
    let length: usize = word_vec.len();
    let mut i = 0;
    while i < length {
        println!("l {:?} {}", word_vec[i], length);
        for letter in word_vec[i].chars() {
            if letter.is_digit(10) {
                println!("letter is {:?} {}", letter, word_vec[i]);
                tuple_vec.push((letter, word_vec[i]));
            }
        }
        i += 1;
    }
    tuple_vec.sort_by_key(|&(key, _)| key);

    for (key, word) in tuple_vec {
        final_string.push_str(word);
        processed_vec.push(word);
        println!("{}: {}", key, word);
    }

    processed_vec.join(" ")
    //loop through elemts
    //take the the order  and rearrange
}

fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_number_vec: Vec<i32> = good
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect();
    let bad_number_vec: Vec<i32> = evil
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect();
    let good_sum: i32 = good_number_vec.iter().sum();
    let bad_sum: i32 = bad_number_vec.iter().sum();

    let good_larger = good_number_vec.len() > bad_number_vec.len();
    let bad_larger = bad_number_vec.len() > good_number_vec.len();
    if bad_sum > good_sum && bad_larger {
        return "Battle Result: Evil eradicates all trace of Good".to_string();
    } else if good_sum > bad_sum && good_larger {
        return "Battle Result: Good triumphs over Evil".to_string();
    } else {
        return "Battle Result: No victor on this battle field".to_string();
    }
}