use std::rc::Rc;
use std::thread;

pub fn rc_fn() {
    let data = Rc::new(String::from("App config"));
    let data_clone = Rc::clone(&data);

    println!("Data: {}, Count: {}", data, Rc::strong_count(&data));
    println!("Data: {}, Count: {}", data_clone, Rc::strong_count(&data));
}

/*
pub fn fc_err() {
    let data = Rc::new(String::from("Oops"));
    let handle = thread::spawn(move || {
        println!("Thread got: {}", data); // 错误：Rc 不是send
    });
    handle.join().unwrap();
}*/
