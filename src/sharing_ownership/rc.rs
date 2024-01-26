use std::rc::Rc;

pub fn shared_rc() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    dbg!(a.as_ptr());
    dbg!(b.as_ptr());
    assert_eq!(a.as_ptr(), b.as_ptr());
}
