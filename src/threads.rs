use std::thread;

pub fn threads() {
    let t1: thread::JoinHandle<()> = thread::spawn(f);
    let t2: thread::JoinHandle<()> = thread::spawn(f);
    println!("Hello from the main thread.");
    t1.join().unwrap();
    t2.join().unwrap();
}
fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
