use std::thread;

pub fn box_leak() {
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    // The move closure might make it look like we’re moving ownership into the threads,
    // but a closer look at the type of x reveals that we’re only giving the threads a reference
    // to the data
    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));
}
