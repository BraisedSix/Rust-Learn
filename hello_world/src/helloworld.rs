pub fn hello() {
    println!("Hello, world!");

    // 变量
    let x = 5; // 不可变
    let mut y = 10; // 可变
    y = 15; // 合法
    println!("x: {},y: {}", x, y);

    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    let guess: u32 = 42; // 类型注解
    let pi = 3.14; // 编译器自动推断为f64
    let heart = "❤️"; // Unicode 字符

    println!("guess:{},pi:{},heart:{}", guess, pi, heart);
}
