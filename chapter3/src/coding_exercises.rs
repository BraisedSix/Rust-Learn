use std::collections::HashMap;

/*
六 . 代码练习
创建一个 Vec<String>，存储 3 个城市名，打印第二个城市
使用 HashMap 存储学生姓名和分数，更新某个学生的分数并打印
编写函数，接受 String，返回其反转形式（如 "hello" -> "olleh"）
编写泛型函数，交换两个值
定义一个 trait Area，为矩形和圆形实现面积计算
创建一个函数，接受两个字符串引用，返回较短的那个，添加生命周期注解
*/

fn shortest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() <= s2.len() {
        s1
    } else {
        s2
    }
}

fn life_fn() {
    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let result = shortest(&s1, &s2);
    println!("Shortest: {}", result);
}

trait Area {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 定义一个 trait Area，为矩形和圆形实现面积计算
fn area_fn() {
    let rect = Rectangle {
        width: 5.0,
        height: 3.0,
    };
    let circle = Circle { radius: 2.0 };

    println!("Rectangle area: {}", rect.area());
    println!("Circle area: {}", circle.area());
}

// 创建一个 Vec<String>，存储 3 个城市名，打印第二个城市

fn create_vec() {
    let vecs: Vec<String> = vec![
        String::from("深圳"),
        String::from("广州"),
        String::from("珠海"),
    ];
    println!("second city:{}", vecs[1]);
}

// 使用 HashMap 存储学生姓名和分数，更新某个学生的分数并打印
fn hashmap_test() {
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert(String::from("张山"), 100);
    map.insert(String::from("里斯"), 120);
    map.insert(String::from("狗不理"), 140);
    println!("{:?}", map);
    map.insert(String::from("狗不理"), 240);
    for (key, value) in &map {
        println!("{}:{}", key, value);
    }
}

// 编写函数，接受 String，返回其反转形式（如 "hello" -> "olleh"）
fn string_rev(s: String) -> String {
    let s2 = s.chars().rev().collect();
    println!("{}", s);
    println!("{}", s2);
    s2
}

// 编写泛型函数，交换两个值
fn gen_fn<T, V>(t: T, v: V) -> (V, T) {
    (v, t)
}

pub fn code_test() {
    create_vec();
    hashmap_test();
    string_rev(String::from("hello"));
    let (x, y) = gen_fn(5, 6);
    println!("{},{}", x, y);
    area_fn();
    life_fn();
}
