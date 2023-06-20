use std::collections::HashMap;
use crypto_hash::{Algorithm, hex_digest};

fn main() {
    // HashMap 사용 예시
    let mut hashmap: HashMap<u32, &str> = HashMap::new();

    hashmap.insert(1, "apple");
    hashmap.insert(2, "banana");
    hashmap.insert(3, "orange");

    let value = hashmap.get(&2);
    println!("Value at key 2: {:?}", value);

    println!("------------------------------------------------");
    HashMapFunction();
    println!("------------------------------------------------");
    HasingFuntion();
}


fn HashMapFunction(){
    let mut scores   = HashMap::new();

    scores.insert(String::from("Blue") ,10);
    scores.insert(String::from("Yellow") ,50);

    for (key, value) in &scores{
        println!("{key}: {value}");
    }

    let team_name = String::from("Blue");
    let score  = scores.get(&team_name).copied().unwrap_or(0);
    println!("Value at key score: {:?}", score);
}


fn HasingFuntion(){
    let input = "Hi Sekai";
    let hash = hex_digest(Algorithm::SHA256, input.as_bytes());
    println!("SHA256 해시: {}", hash);
}








