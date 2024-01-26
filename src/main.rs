mod sharing_ownership;
mod threads;

fn main() {
    // threads::thread_func::thread_func();
    // threads::thread_closure::thread_closure();
    // threads::scoped_threads::scoped_thread();
    // sharing_ownership::static_sharing::static_sharing();
    // sharing_ownership::box_leak::box_leak();
    sharing_ownership::rc::shared_rc();
}
