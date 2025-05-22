use std::thread;

pub fn copy_fn() {
    let number = 42;

    let handle = thread::spawn(move || {
        println!("Thread got: {} 🚀", number);
    });

    println!("Main: {}", number);
    handle.join().unwrap();
}
