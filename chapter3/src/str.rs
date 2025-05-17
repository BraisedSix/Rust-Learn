/*
 字符串处理
Rust 的字符串有两种类型：

&str：字符串切片，引用不可变数据
String：堆分配，可变字符串
format! 类似 println!，但返回 String
字符串切片需注意 UTF-8 边界
push_str 和 push 修改 String
replace 替换子字符串

*/

pub fn str_fn() {
    let s1 = String::from("hello"); // String
    let s2 = "world"; // &str
    let s3 = s2.to_string(); // &str 转为 String

    let combined = format!("{} {}", s1, s2); // 拼接
    println!("{}", combined);

    let slice = &combined[0..5]; //切片
    println!("Slice: {}", slice);

    let mut s = String::from("hello");
    s.push_str(", world"); // 添加字符串
    s.push('👋'); // 添加字符
    println!("{}", s);

    let replaced = s.replace("hello", "hi");
    println!("Replaced:{}", replaced);
}
