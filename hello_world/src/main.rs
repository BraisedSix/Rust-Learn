// 这里就是第二章讲的模块系统中的文件模块
mod chapter2;
mod helloworld;

fn main() {
    helloworld::hello();
    chapter2::chapter2_fn();
}
