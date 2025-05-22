use std::sync::mpsc;
use std::thread;

/*
使用 通道（channel） 进行线程间通信：一个线程（生产者）发送消息，
另一个线程（消费者，通常是主线程）接收消息
tx 移到线程，rx 留给主线程。
场景：服务器线程发送处理结果给主线程。
*/
pub fn channel_fn() {
    // 作用：创建了一个 多生产者，单消费者（Multi-Producer, Single-Consumer） 通道，返回发送端（tx）和接收端（rx）。
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("Task completed!");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Main got: {} 🔥", received);
}
