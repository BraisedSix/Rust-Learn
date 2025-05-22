use std::thread;
/*
match 覆盖 Ok 和 Err
场景：处理任务结果
*/
pub fn match_fn() {
    let handle = thread::spawn(|| -> Result<String, &str> { Ok("Task completed!".to_string()) });

    match handle.join().unwrap() {
        Ok(msg) => println!("Thread success: {}  🚀", msg),
        Err(e) => println!("Thread failed: {}", e),
    }
    // Thread success: Task completed!  🚀
}
enum TaskStatus {
    Completed(u32),
    Pending,
}

/*
if let 提取 Completed 的 ID
场景：检查任务状态
*/
pub fn if_let() {
    let status = TaskStatus::Completed(100);
    if let TaskStatus::Completed(id) = status {
        println!("Task completed with ID: {} 🚀", id);
    } else {
        println!("Task still pending...");
    }
    // Task completed with ID: 100 🚀
}

struct Task {
    id: i32,
    priority: i32,
}

/*
模式匹配支持解构，@ 绑定捕获值
let Task { id, priority }：解构结构体
p @ 1..=10：捕获优先级
场景：解析任务数据
*/
pub fn dec_fn() {
    let task = Task { id: 1, priority: 5 };
    let Task { id, priority } = task;
    println!("ID: {}, Priority: {}", id, priority);
    match task {
        Task {
            id,
            priority: p @ 1..=10,
        } => println!("High priority task {}: priority {}", id, p),
        Task { id, priority } => println!("Task {}: priority {}", id, priority),
    }

    /*
    ID: 1, Priority: 5
    High priority task 1: priority 5
    */
}

pub fn match_thread() {
    let handle = thread::spawn(|| -> Option<i32> {
        Some(42) //模拟计算结果
    });

    match handle.join().unwrap() {
        Some(value) => println!("Result: {}  🚀", value),
        None => println!("No result found"),
    }
    // Result: 42  🚀
}
