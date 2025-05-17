/*
  三 . 泛型
泛型允许函数或结构体处理多种类型
T: PartialOrd 约束 T 必须支持比较
泛型可用于结构体和枚举
*/

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn generics_fn() {
    let numbers = vec![34, 50, 25, 100];
    println!("Largest: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest: {}", largest(&chars));
}
