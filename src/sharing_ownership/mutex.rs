use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn mutex() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        // we spawn ten threads
        // to each increment the integer one hundred times
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                // drop the guard before sleeping!
                drop(guard);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    // The into_inner method takes ownership of the mutex,
    // which guarantees that nothing else can have a reference to the mutex anymore,
    // making locking unnecessary.
    // assert_eq!(n.into_inner().unwrap(), 1000);
    dbg!(n.into_inner().unwrap());
}
