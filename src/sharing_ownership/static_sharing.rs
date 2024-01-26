use std::thread;
// A static item has a constant initializer, is never dropped, and already exists before
// the main function of the program even starts. Every thread can borrow it, since it’s
// guaranteed to always exist.
static X: [i32; 3] = [1, 2, 3];

pub fn static_sharing() {
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));
}
