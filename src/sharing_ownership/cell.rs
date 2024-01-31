use std::cell::Cell;

// pub fn f(a: &Cell<i32>, b: &Cell<i32>) {
//     let before = a.get();
//     b.set(b.get() + 1);
//     let after = a.get();
//     if before != after {
//         // x(); // might happen
//     }
// }

pub fn f(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take(); // Replaces the contents of the Cell with an empty Vec
    v2.push(1);
    v.set(v2); // Put the modified Vec back
}
