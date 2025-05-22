mod arct;
mod boxt;
mod channel;
mod copy;
mod matcht;
mod move_closure;
mod mutex_arc;
mod practise;
mod rct;
mod thread;

/*
Arc：多线程共享 Mutex。
Mutex::lock：一次只允许一个线程修改。
场景：统计服务器任务数或文件处理计数。
*/

fn main() {
    /*thread::thread_fn();
    move_closure::move_fn();
    copy::copy_fn();
    channel::channel_fn();
    mutex_arc::mutex_arc();
    boxt::box_fn();
    boxt::box_fn2();
    arct::arc_fn();
    rct::rc_fn();
    matcht::match_fn();
    matcht::if_let();
    matcht::dec_fn();
    matcht::match_thread();*/
    practise::practise_fn();
}
