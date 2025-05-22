/*
创建两个线程，一个打印 1-10 奇数，一个打印偶数，用 join 确保完成
用通道实现生产者-消费者：生产者发送 1-5，消费者打印
用 Arc<Mutex<Vec<i32>>> 实现 3 个线程添加数字，打印最终列表
用 Box 实现任务链表，添加插入方法，打印任务
用 match 处理线程返回的 Result<i32, &str>，打印成功或错误
用 if let 处理 Option<Task>，打印 ID 或“None”
*/

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

// 创建两个线程，一个打印 1-10 奇数，一个打印偶数，用 join 确保完成
fn two_thread() {
    let mut handles = vec![];
    for i in 1..11 {
        let handle;
        if i % 2 == 0 {
            handle = thread::spawn(move || {
                println!("偶数：{}", i);
            });
        } else {
            handle = thread::spawn(move || {
                println!("奇数：{}", i);
            });
        }
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// 用通道实现生产者-消费者：生产者发送 1-5，消费者打印
fn channel_fn() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(vec![1, 2, 3, 4, 5]).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("received:{:?}", received);
    for data in &received {
        println!("{}", data);
    }
}

// 用 Arc<Mutex<Vec<i32>>> 实现 3 个线程添加数字，打印最终列表
fn arc_fn() {
    let numbers = Arc::new(Mutex::new(vec![1]));
    let mut handles = vec![];
    for i in 0..3 {
        let number = Arc::clone(&numbers);
        let handle = thread::spawn(move || {
            let mut num = number.lock().unwrap();
            num.push(i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("numbers: {:?}", numbers.lock().unwrap());
    /*
    交替执行
     numbers: [1, 0, 2, 1]
     numbers: [1, 0, 1, 2]
    */
}

#[derive(Debug)]
enum TaskList {
    Cons(String, Box<TaskList>),
    Nil,
}

impl TaskList {
    // 插入新任务到链表末尾
    fn insert(&mut self, task_name: String) {
        match self {
            TaskList::Cons(_, next) => {
                if matches!(**next, TaskList::Nil) {
                    *next = Box::new(TaskList::Cons(task_name, Box::new(TaskList::Nil)));
                } else {
                    next.insert(task_name);
                }
            }
            TaskList::Nil => {
                *self = TaskList::Cons(task_name, Box::new(TaskList::Nil));
            }
        }
    }

    // 打印所有任务
    fn print_tasks(&self) {
        match self {
            TaskList::Cons(name, next) => {
                println!("Task: {}", name);
                next.print_tasks();
            }
            TaskList::Nil => println!("End of tasks"),
        }
    }
}

fn task_add() {
    // 单线程测试
    let mut list = TaskList::Nil;
    list.insert("Task 1".to_string());
    list.insert("Task 2".to_string());
    list.insert("Task 3".to_string());
    println!("Single-threaded tasks:");
    println!("list: {:?}", list);
    list.print_tasks();

    // 并发测试
    let list = Arc::new(Mutex::new(TaskList::Nil));
    let mut handles = vec![];

    // 多个线程插入任务
    for i in 1..=3 {
        let list = Arc::clone(&list);
        let handle = thread::spawn(move || {
            let mut list = list.lock().unwrap();
            list.insert(format!("Task {} (thread)", i));
        });
        handles.push(handle);
    }

    // 等待线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nConcurrent tasks:");
    list.lock().unwrap().print_tasks();
}

// 用 match 处理线程返回的 Result<i32, &str>，打印成功或错误
fn match_fn() {
    let handle = thread::spawn(move || -> Result<i32, &str> { Ok(12) });
    match handle.join().unwrap() {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    }
}

#[derive(Debug)]
struct Task {
    id: i32,
    priority: i32,
}

// 用 if let 处理 Option<Task>，打印 ID 或“None”
fn if_let_fn() {
    let task = Some(Task { id: 1, priority: 5 });
    let no_task: Option<Task> = None;

    println!("ID:{:?}", task);
    println!("None:{:?}", no_task);
}

pub fn practise_fn() {
    //two_thread();
    //channel_fn();
    //arc_fn();
    //task_add();
    //match_fn();
    if_let_fn();
}
