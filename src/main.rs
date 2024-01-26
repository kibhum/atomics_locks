mod threads;

fn main() {
    threads::thread_func::thread_func();
    threads::thread_closure::thread_closure();
    threads::scoped_threads::scoped_thread();
}
