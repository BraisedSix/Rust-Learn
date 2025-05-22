use std::thread;

/*
move ||：闭包拿走 data 所有权
主线程失去 data 访问权，防竞争
场景：服务器用 move 传递请求数据给线程

*/

pub fn move_fn() {
    let data = String::from("Hello, Rust!");
    let handle = thread::spawn(move || {
        println!("Thread got: {} 🚀", data);
    });

    // 错误：data 已经移动
    //  println!("Main: {}", data);

    handle.join().unwrap();
}
