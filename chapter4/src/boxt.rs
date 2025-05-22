use std::thread;

#[derive(Debug)]

/*
构造一个任务链表 存储任务列表并打印出任务列表（如服务器任务队列）
*/
enum TaskList {
    Cons(String, Box<TaskList>),
    Nil,
}

pub fn box_fn() {
    let list = TaskList::Cons(
        "Task 1".to_string(),
        Box::new(TaskList::Cons(
            "Task 2".to_string(),
            Box::new(TaskList::Nil),
        )),
    );
    println!("Task list: {:?}", list);
    // Task list: Cons("Task 1", Cons("Task 2", Nil))
}

pub fn box_fn2() {
    let handle = thread::spawn(|| {
        let big_data = Box::new(vec![1; 1000]); //堆上大数组
        println!("Thread processing {} items", big_data.len());
    });
    handle.join().unwrap();
}
