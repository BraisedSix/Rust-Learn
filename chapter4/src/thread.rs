use std::thread;
use std::time::Duration;

pub fn thread_fn() {
    // 创建线程
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread says: {} 🚀", i);
            // 模拟耗时任务
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 主线程干点啥
    for i in 1..3 {
        println!("Main says: {} 🔥", i);
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap(); // 等待线程完成
}
