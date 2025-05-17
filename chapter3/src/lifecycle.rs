/*
五 . 生命周期
生命周期确保引用的有效性
'a 表示引用共享同一生命周期
结构体中的生命周期
*/

struct Excerpt<'a> {
    part: &'a str,
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn life_cycle() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(s1.as_str(), s2);
    println!("Longest: {}", result);

    let novel = String::from("Call me Ishmael...");
    let first = Excerpt { part: &novel[..10] };
    println!("Excerpt: {}", first.part);
}
