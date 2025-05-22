use std::sync::{Arc, Mutex};
use std::thread;

/*
 任务计数 统计多线程处理的任务数
3 个线程共享计数器，递增后结果为 3

Arc::new(Mutex::new(0))：创建共享计数器，Mutex 保护 i32，Arc 允许多线程访问

Arc::clone(&counter)：为每个线程克隆 Arc 引用，计数加 1

move ||：线程拥有 Arc>，通过 lock() 修改

Arc 管理引用计数，线程销毁时计数减 1，计数为 0 时释放数据

Mutex 确保一次只有一个线程修改，防止数据竞争

场景：服务器统计任务数
*/

pub fn arc_fn() {
    let task_count = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..3 {
        let task_count = Arc::clone(&task_count);
        let handle = thread::spawn(move || {
            let mut count = task_count.lock().unwrap();
            *count += 1;
            println!("Thread {} completed task", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total tasks: {}", *task_count.lock().unwrap());
    /*
    Thread 0 completed task
    Thread 1 completed task
    Thread 2 completed task
    Total tasks: 3
         */
}
