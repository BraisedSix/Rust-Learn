pub fn vec() {
    let mut v: Vec<i32> = Vec::new(); // 空向量
    v.push(1);
    v.push(2);
    println!("Vector:{:?}", v); // 对象使用 {:?} 打印

    let v2 = vec![1, 2, 3]; // 使用宏命令初始化
    let second = v2[1]; //索引访问
    println!("{}", second);
    // 安全访问
    if let Some(val) = v2.get(1) {
        println!("Second: {}", val);
    }
    v.pop(); //移除末尾元素
    for i in &v {
        println!("{}", i);
    }
}
