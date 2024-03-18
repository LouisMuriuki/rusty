use std::collections::HashMap;
fn main() {
    let mut scores =HashMap::new();

    let selectedTeam="Blue".to_string();
    println!("Hello, world!");
    scores.insert("Team A".to_string(),20);
    scores.insert("Blue".to_string(),40);
    scores.insert("Team C".to_string(),50);

    println!("{:?}",&scores);

    let score=scores.get(&selectedTeam).copied().unwrap_or(0);
    println!("{:?}",score);

    iterateHashMaps(scores);

}

fn iterateHashMaps(map:HashMap<String,i32>){
    for (key,value) in map{
        println!("{key}:{value},")
    }
}
