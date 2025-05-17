use std::{fs::File, io::Read};

pub fn chapter2_fn() {
    //  所有权
    /*示例：移动 */
    let s1 = String::from("hello"); // s1是堆上的字符串
    let s2 = s1; // s1 被移动到s2, s1失效
    println!("{}", s2); // 合法
                        // println!("{}", s1); // 错误 s1已失效
                        /*示例：移动 */

    /*示例：借用 */
    let s = String::from("hello");
    print_lenght(&s); // 传递引用
    println!("{}", s2); // s 仍可用
                        /*示例：借用 */

    //  数据类型
    /* 标量类型 */
    let pi: f64 = 3.14159; // 双精度浮点数
    let half: f32 = 0.5; // 单精度浮点数
    let sum = pi + half as f64; // 类型转换后相加
    println!("Pi: {}, Half: {}, Sum: {}", pi, half, sum);

    // 这里可以加上类型，不加的话会自动推导出类型
    let is_rust_cool = true; // Rust 当然很酷！
    let is_learning = false; // 你会改成 true 的，对吧？
    if is_rust_cool {
        println!("Rust is awesome!");
    } else {
        println!("Give Rust a chance!");
    }
    println!("Learning: {}", is_learning);

    let letter = 'A'; // 英文字符
    let hanzi = '锈'; // 中文字符（Rust 的“锈”）
    let emoji = '🚀'; // 表情符号
    println!("Letter: {}, Hanzi: {}, Emoji: {}", letter, hanzi, emoji);
    /* 标量类型 */

    /* 复合类型 */
    // 元组：固定长度，异构数据
    let tup: (i32, f64, char) = (500, 6.4, 'a');
    let (x, y, z) = tup; // 解构
    println!("y: {}", y);
    println!("Second element: {}", tup.1); // 索引访问

    // 数组：固定长度，同构数据
    let arr: [i32; 3] = [1, 2, 3]; // 3 个相同的数据类型
    println!("First: {}", arr[0]);

    /* 复合类型 */

    /* 函数 */
    let sum = add(5, 10);
    println!("Sum: {}", sum);
    /* 函数 */

    /* 控制流 */
    //  if 表达式
    let number = 7;
    let result = if number % 2 == 0 { "Even" } else { "Odd" };
    println!("Number is {}", result);

    // 循环
    // loop：无限循环，直到 break
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 3 {
            break count * 2;
        }
    };
    println!("Result: {}", result); // 输出 6

    // while：条件循环
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    // for：迭代集合
    let arr = [10, 20, 30];
    for elem in arr.iter() {
        println!("{}", elem);
    }
    /* 控制流 */

    /* 模块使用 */
    println!("Square of 4: {}", math::square(4));
    // 使用 use访问
    use math::square;
    println!("Square of 4: {}", square(4));
    /* 模块使用 */

    /* 错误处理 Rust 使用 Option 和 Result 处理潜在错误，避免传统异常 */
    // Option
    let numbers = vec![1, 2, 3];
    match first_element(&numbers) {
        Some(val) => println!("First: {}", val),
        None => println!("Empty array"),
    }
    // Result
    match open_file("test.txt") {
        Ok(file) => println!("File opened: {:?}", file),
        Err(e) => println!("Error: {}", e),
    }

    // ? 运算符 ,简化错误传播
    match read_file("test.txt") {
        Ok(contents) => println!("Contents: {}", contents),
        Err(e) => println!("Error: {}", e),
    }

    /* 错误处理 */
}

// 函数使用 fn 定义，支持参数和返回值
fn add(a: i32, b: i32) -> i32 {
    a + b // 无分号表示返回值
}

fn print_lenght(s: &String) {
    println!("Lenght: {}", s.len());
}

//1. Option 表示值可能存在或不存在
fn first_element(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        None
    } else {
        Some(arr[0])
    }
}

//  2. Result 表示操作成功或失败
fn open_file(path: &str) -> Result<File, std::io::Error> {
    File::open(path)
}

// ? 运算符 ,简化错误传播
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 模块系统
// 定义模块
mod math {
    pub fn square(num: i32) -> i32 {
        num * num
    }
}
