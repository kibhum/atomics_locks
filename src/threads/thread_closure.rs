use std::thread;

pub fn thread_closure() {
    // let numbers = vec![1, 2, 3];
    // thread::spawn(move || {
    //     for n in numbers {
    //         println!("{n}");
    //     }
    // })
    // .join()
    // .unwrap();

    let numbers = Vec::from_iter(0..=100);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();
    println!("average: {average}");
}
