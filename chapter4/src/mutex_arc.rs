use std::sync::{Arc, Mutex};
use std::thread;

/*
std::sync::Arc：提供 原子引用计数（Atomic Reference Counting），允许多线程共享数据
std::sync::Mutex：提供 互斥锁（Mutual Exclusion），确保一次只有一个线程修改共享数据
std::thread：提供线程功能（如 thread::spawn）
Mutex::new(0)：创建互斥锁，包裹值 0（类型 i32），类型是 Mutex<i32>
Mutex::lock：一次只允许一个线程修改
Arc::new(...)：将 Mutex<i32> 包裹在 Arc 中，类型是 Arc<Mutex<i32>>
counter：绑定到这个共享对象
场景：统计服务器任务数或文件处理计数
*/
pub fn mutex_arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
    // Final count: 5
}
