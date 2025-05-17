/*
四 . Trait
Trait 定义共享行为，类似接口：
*/

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{},{}", self.headline, self.content)
    }
}

pub fn trait_fn() {
    let article = Article {
        headline: String::from("Rust News"),
        content: String::from("Rust 1.82 released!"),
    };
    println!("{:?}", article.summarize());
}
