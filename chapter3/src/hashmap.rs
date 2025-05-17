use std::collections::HashMap;
/*
HashMap<K, V>
键值对存储，支持快速查找
*/

pub fn hashmap_fn() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    if let Some(score) = scores.get("Blue") {
        println!("Blue's score: {}", score);
    }
    // 在外面无法访问
    // println!("Blue's score2: {}", score);

    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }
    // entry 处理键不存在的情况
    scores.entry(String::from("Yellow")).or_insert(30);
    println!("{:?}", scores);
}
