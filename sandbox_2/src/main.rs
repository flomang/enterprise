use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for letter in text.chars() {
        let count = map.entry(letter).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
